use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

use windows::Devices::Enumeration::{DeviceClass, DeviceInformation};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::Imaging::{BitmapPixelFormat, SoftwareBitmap};
use windows::Media::Capture::{
    MediaCapture, MediaCaptureInitializationSettings, MediaCaptureMemoryPreference,
    StreamingCaptureMode,
};
use windows::Media::Capture::Frames::{
    MediaFrameArrivedEventArgs, MediaFrameReader, MediaFrameSource, MediaFrameSourceGroup,
};
use windows::Storage::Streams::{Buffer, DataReader};

pub struct WmfCapture {
    media_capture: Option<MediaCapture>,
    reader: Option<MediaFrameReader>,
    running: Arc<AtomicBool>,
    last_frame: Arc<Mutex<Option<Vec<u8>>>>,
    frame_width: Arc<Mutex<u32>>,
    frame_height: Arc<Mutex<u32>>,
}

impl WmfCapture {
    pub fn new() -> Self {
        Self {
            media_capture: None,
            reader: None,
            running: Arc::new(AtomicBool::new(false)),
            last_frame: Arc::new(Mutex::new(None)),
            frame_width: Arc::new(Mutex::new(0)),
            frame_height: Arc::new(Mutex::new(0)),
        }
    }

    pub fn enumerate() -> Vec<(String, String)> {
        info!("Enumerating camera devices via WMF");
        let op = match DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::VideoCapture) {
            Ok(o) => o,
            Err(e) => {
                warn!("Failed to query devices: {}", e);
                return vec![];
            }
        };

        let devices = match op.get() {
            Ok(d) => d,
            Err(e) => {
                warn!("Failed to get device list: {}", e);
                return vec![];
            }
        };

        devices
            .into_iter()
            .filter_map(|d| {
                let id = d.Id().ok()?;
                let name = d.Name().ok()?;
                Some((id.to_string(), name.to_string()))
            })
            .collect()
    }

    pub fn start(&mut self, device_id: &str) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("WMF capture already running".to_string());
        }
        self.running.store(true, Ordering::SeqCst);

        let capture = MediaCapture::new()
            .map_err(|e| format!("Failed to create MediaCapture: {}", e))?;

        let all_groups = MediaFrameSourceGroup::FindAllAsync()
            .map_err(|e| format!("Failed to find groups: {}", e))?
            .get()
            .map_err(|e| format!("FindAllAsync: {}", e))?;

        info!("Found {} source groups", all_groups.Size().unwrap_or(0));
        let mut first_group = None;
        let mut selected_group = None;
        for g in all_groups.into_iter() {
            let gid: String = g.Id().ok().map(|s| s.to_string()).unwrap_or_default();
            let gname: String = g.DisplayName().ok().map(|s| s.to_string()).unwrap_or_default();
            info!("  Group: {} ({})", gname, gid);
            if first_group.is_none() {
                first_group = Some(g.clone());
            }
            if gid.contains(device_id) || gname.contains(device_id) {
                info!("  -> Matched device");
                selected_group = Some(g);
            }
        }
        if selected_group.is_none() && device_id == "default" {
            if let Some(ref group) = first_group {
                info!("  -> Using first group as default");
                selected_group = Some(group.clone());
            }
        }

        let settings = MediaCaptureInitializationSettings::new()
            .map_err(|e| format!("Failed to create settings: {}", e))?;
        if let Some(ref group) = selected_group {
            settings.SetSourceGroup(group)
                .map_err(|e| format!("Failed to set source group: {}", e))?;
            info!("Using source group for init");
        } else {
            settings.SetVideoDeviceId(&windows::core::HSTRING::from(device_id))
                .map_err(|e| format!("Failed to set device id: {}", e))?;
            info!("Using device id for init (no group match)");
        }
        settings.SetStreamingCaptureMode(StreamingCaptureMode::Video)
            .map_err(|e| format!("Failed to set capture mode: {}", e))?;
        settings.SetMemoryPreference(MediaCaptureMemoryPreference::Cpu)
            .map_err(|e| format!("Failed to set memory: {}", e))?;

        info!("Initializing MediaCapture...");
        capture.InitializeWithSettingsAsync(&settings)
            .map_err(|e| format!("Init failed: {}", e))?
            .get()
            .map_err(|e| format!("Async init: {}", e))?;

        let frame_sources = capture.FrameSources()
            .map_err(|e| format!("FrameSources error: {}", e))?;
        info!("FrameSources size: {}", frame_sources.Size().unwrap_or(0));

        let frame_source: Option<MediaFrameSource> = if let Some(ref group) = selected_group {
            let infos = group.SourceInfos()
                .map_err(|e| format!("SourceInfos: {}", e))?;
            let mut found = None;
            for info in infos.into_iter() {
                if let Ok(kind) = info.SourceKind() {
                    let kind_str = format!("{:?}", kind);
                    info!("  SourceInfo kind: {}", kind_str);
                    if let Ok(src_id) = info.Id() {
                        let sid: String = src_id.to_string();
                        info!("  SourceInfo id: {}", sid);
                        if let Ok(source) = frame_sources.Lookup(&src_id) {
                            info!("  -> Found by Lookup!");
                            found = Some(source);
                        } else {
                            warn!("  Lookup failed for id: {}", sid);
                        }
                    }
                }
            }
            found
        } else {
            // Fallback: iterate manually
            let iter = frame_sources.First()
                .map_err(|e| format!("Iterator: {}", e))?;
            let mut found = None;
            while iter.MoveNext().map_err(|e| format!("MoveNext: {}", e))? {
                if let Ok(kv) = iter.Current() {
                    if let Ok(source) = kv.Value() {
                        found = Some(source);
                        break;
                    }
                }
            }
            found
        };

        let frame_source = frame_source.ok_or_else(|| {
            let msg = "No frame source found".to_string();
            warn!("{}", msg);
            msg
        })?;

        info!("Creating frame reader");
        let reader = capture
            .CreateFrameReaderAsync(&frame_source)
            .map_err(|e| format!("CreateFrameReader: {}", e))?
            .get()
            .map_err(|e| format!("Reader creation: {}", e))?;

        let lf = self.last_frame.clone();
        let fw = self.frame_width.clone();
        let fh = self.frame_height.clone();
        let running = self.running.clone();

        let handler = TypedEventHandler::<MediaFrameReader, MediaFrameArrivedEventArgs>::new(
            move |sender: &Option<MediaFrameReader>,
                  _args: &Option<MediaFrameArrivedEventArgs>| {
                if !running.load(Ordering::SeqCst) {
                    return Ok(());
                }
                if let Some(sender) = sender {
                    if let Ok(frame_ref) = sender.TryAcquireLatestFrame() {
                        if let Ok(video_frame) = frame_ref.VideoMediaFrame() {
                            if let Ok(bitmap) = video_frame.SoftwareBitmap() {
                                if let Some(rgba) = software_bitmap_to_rgba(&bitmap) {
                                    *fw.lock().unwrap() = bitmap.PixelWidth().unwrap_or(0) as u32;
                                    *fh.lock().unwrap() = bitmap.PixelHeight().unwrap_or(0) as u32;
                                    *lf.lock().unwrap() = Some(rgba);
                                }
                            }
                        }
                    }
                }
                Ok(())
            },
        );

        reader.FrameArrived(&handler)
            .map_err(|e| format!("Set handler: {}", e))?;

        reader.StartAsync()
            .map_err(|e| format!("Start reader: {}", e))?
            .get()
            .map_err(|e| format!("StartAsync: {}", e))?;

        self.media_capture = Some(capture);
        self.reader = Some(reader);
        info!("WMF camera capture started successfully");
        Ok(())
    }

    pub fn stop(&mut self) {
        info!("Stopping WMF capture");
        self.running.store(false, Ordering::SeqCst);
        if let Some(ref reader) = self.reader {
            if let Ok(action) = reader.StopAsync() {
                let _ = action.get();
            }
        }
        self.media_capture = None;
        self.reader = None;
    }

    pub fn try_read_frame(&self) -> Option<(Vec<u8>, u32, u32)> {
        let data = self.last_frame.lock().ok()?.take();
        let w = *self.frame_width.lock().ok()?;
        let h = *self.frame_height.lock().ok()?;
        if w > 0 && h > 0 {
            data.map(|d| (d, w, h))
        } else {
            None
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

fn software_bitmap_to_rgba(bitmap: &SoftwareBitmap) -> Option<Vec<u8>> {
    let w = bitmap.PixelWidth().ok()? as usize;
    let h = bitmap.PixelHeight().ok()? as usize;
    let fmt = bitmap.BitmapPixelFormat().ok()?;

    let target = if fmt == BitmapPixelFormat::Rgba8 {
        bitmap.clone()
    } else {
        SoftwareBitmap::Convert(bitmap, BitmapPixelFormat::Rgba8).ok()?
    };

    let buf = Buffer::Create((w * h * 4) as u32).ok()?;
    target.CopyToBuffer(&buf).ok()?;
    let reader = DataReader::FromBuffer(&buf).ok()?;
    let mut bytes = vec![0u8; w * h * 4];
    reader.ReadBytes(&mut bytes).ok()?;
    Some(bytes)
}
