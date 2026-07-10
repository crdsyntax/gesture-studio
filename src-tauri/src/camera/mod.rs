pub mod commands;
#[cfg(windows)]
pub mod wmf;

use crate::events::AppEvent;
use crate::models::CameraStatus;
use crate::app::events_bridge;
use crossbeam_channel::Sender;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Instant;
use tracing::{info, error, warn};

pub struct CameraEngine {
    running: Arc<AtomicBool>,
    event_tx: Sender<AppEvent>,
    status: CameraStatus,
    fps: Arc<AtomicU32>,
}

impl CameraEngine {
    pub fn new(event_tx: Sender<AppEvent>) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            event_tx,
            status: CameraStatus::Idle,
            fps: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn start(&mut self, device: &str, target_fps: u32) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Camera already running".to_string());
        }

        self.running.store(true, Ordering::SeqCst);
        self.status = CameraStatus::Starting;
        info!("Starting camera: {} at {} FPS", device, target_fps);

        let running = self.running.clone();
        let event_tx = self.event_tx.clone();
        let _device = device.to_string();
        let fps = self.fps.clone();

        std::thread::spawn(move || {
            let _ = event_tx.send(AppEvent::CameraStarted { device: _device.clone() });
            info!("Camera thread started");

            #[cfg(windows)]
            {
                if start_wmf_camera(&running, &event_tx, &_device, target_fps, &fps) {
                    return;
                }
                info!("WMF capture not available, falling back to synthetic");
            }

            run_synthetic_camera(&running, &event_tx, target_fps, &fps);
        });

        self.status = CameraStatus::Running;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.status = CameraStatus::Stopped;
        info!("Camera stopping");
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn get_fps(&self) -> u32 {
        self.fps.load(Ordering::SeqCst)
    }
}

struct EncodeTask {
    rgb: Vec<u8>,
    width: u32,
    height: u32,
}

fn spawn_encoder_thread(rx: mpsc::Receiver<EncodeTask>) {
    std::thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            let jpeg = compress_to_jpeg(&task.rgb, task.width, task.height);
            let b64 = base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                &jpeg,
            );
            events_bridge::emit_frame(&b64, task.width, task.height);
        }
    });
}

#[cfg(windows)]
fn start_wmf_camera(
    running: &Arc<AtomicBool>,
    event_tx: &Sender<AppEvent>,
    device_id: &str,
    target_fps: u32,
    fps_counter: &Arc<AtomicU32>,
) -> bool {
    use crate::camera::wmf::WmfCapture;
    use std::time::Duration;

    let devices = WmfCapture::enumerate();
    if devices.is_empty() {
        info!("No WMF camera devices found");
        return false;
    }

    info!("Found {} camera device(s)", devices.len());
    let real_id = if device_id == "default" || device_id.is_empty() {
        devices[0].0.as_str()
    } else {
        device_id
    };

    let mut wmf = WmfCapture::new();
    if let Err(e) = wmf.start(real_id) {
        warn!("Failed to start WMF camera: {}", e);
        return false;
    }

    let (encode_tx, encode_rx) = mpsc::channel::<EncodeTask>();
    spawn_encoder_thread(encode_rx);

    let frame_interval_ms = if target_fps > 0 { 1000 / target_fps } else { 33 };
    let frame_interval = Duration::from_millis(frame_interval_ms as u64);
    let mut last_frame_time = std::time::Instant::now();

    let mut frame_count: u32 = 0;
    let mut fps_timer = std::time::Instant::now();

    while running.load(Ordering::SeqCst) {
        if let Some((raw_data, width, height)) = wmf.try_read_frame() {
            if last_frame_time.elapsed() >= frame_interval {
                last_frame_time = std::time::Instant::now();

                // WMF produces RGBA (4bpp), pipeline expects RGB (3bpp)
                let rgb_data = if raw_data.len() == (width * height * 4) as usize {
                    rgba_to_rgb(&raw_data)
                } else {
                    raw_data.clone()
                };

                let frame = crate::models::CameraFrame {
                    timestamp: chrono::Utc::now(),
                    data: rgb_data.clone(),
                    width,
                    height,
                };
                let _ = event_tx.send(AppEvent::NewFrame { frame });

                // Offload JPEG encoding to background thread
                let _ = encode_tx.send(EncodeTask {
                    rgb: rgb_data,
                    width,
                    height,
                });

                // Track FPS every second
                frame_count += 1;
                if fps_timer.elapsed() >= Duration::from_secs(1) {
                    fps_counter.store(frame_count, Ordering::SeqCst);
                    frame_count = 0;
                    fps_timer = std::time::Instant::now();
                }
            }
        } else {
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    drop(encode_tx);
    wmf.stop();
    let _ = event_tx.send(AppEvent::CameraStopped);
    info!("WMF camera thread stopped");
    true
}

fn run_synthetic_camera(
    running: &Arc<AtomicBool>,
    event_tx: &Sender<AppEvent>,
    target_fps: u32,
    fps_counter: &Arc<AtomicU32>,
) {
    let width = 320u32;
    let height = 240u32;
    let start = Instant::now();
    let frame_interval_ms = if target_fps > 0 { 1000 / target_fps } else { 33 };
    let frame_interval = std::time::Duration::from_millis(frame_interval_ms as u64);

    let (encode_tx, encode_rx) = mpsc::channel::<EncodeTask>();
    spawn_encoder_thread(encode_rx);

    let mut frame_count: u32 = 0;
    let mut fps_timer = std::time::Instant::now();

    while running.load(Ordering::SeqCst) {
        let elapsed = start.elapsed().as_secs_f64();

        let frame_data = generate_synthetic_frame(width, height, elapsed);
        let frame = crate::models::CameraFrame {
            timestamp: chrono::Utc::now(),
            data: frame_data.clone(),
            width,
            height,
        };

        let _ = event_tx.send(AppEvent::NewFrame { frame });

        // Offload JPEG encoding to background thread
        let _ = encode_tx.send(EncodeTask {
            rgb: frame_data,
            width,
            height,
        });

        // Track FPS every second
        frame_count += 1;
        if fps_timer.elapsed() >= std::time::Duration::from_secs(1) {
            fps_counter.store(frame_count, Ordering::SeqCst);
            frame_count = 0;
            fps_timer = std::time::Instant::now();
        }

        std::thread::sleep(frame_interval);
    }

    drop(encode_tx);
    let _ = event_tx.send(AppEvent::CameraStopped);
    info!("Synthetic camera thread stopped");
}

fn rgba_to_rgb(rgba: &[u8]) -> Vec<u8> {
    rgba.chunks_exact(4).flat_map(|c| c[..3].iter().copied()).collect()
}

fn generate_synthetic_frame(width: u32, height: u32, time: f64) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            let r = ((x as f64 / width as f64 * 180.0 + time * 40.0) % 256.0) as u8;
            let g = ((y as f64 / height as f64 * 180.0 + time * 25.0) % 256.0) as u8;
            let b = (128.0 + (time * 0.8).sin() * 100.0) as u8;
            data.push(r);
            data.push(g);
            data.push(b);
        }
    }
    data
}

fn compress_to_jpeg(rgb: &[u8], width: u32, height: u32) -> Vec<u8> {
    use image::EncodableLayout;
    let img = image::RgbImage::from_raw(width, height, rgb.to_vec())
        .unwrap_or_else(|| image::RgbImage::new(width, height));
    let mut jpeg_buf = std::io::Cursor::new(Vec::new());
    if let Err(e) = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_buf, 60)
        .encode(img.as_bytes(), width, height, image::ExtendedColorType::Rgb8)
    {
        error!("JPEG encoding failed: {}", e);
        return rgb.to_vec();
    }
    jpeg_buf.into_inner()
}
