pub mod app;
pub mod camera;
pub mod vision;
pub mod landmarks;
pub mod gestures;
pub mod trainer;
pub mod actions;
pub mod storage;
pub mod models;
pub mod services;
pub mod config;
pub mod utils;
pub mod events;

pub use config::AppConfig;
pub use events::AppEvent;
pub use models::*;

use services::GestureService;
use std::sync::{Arc, Mutex, RwLock};
use tracing::info;

pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub event_tx: crossbeam_channel::Sender<AppEvent>,
    pub storage: Arc<Mutex<storage::Storage>>,
    pub service: Arc<Mutex<GestureService>>,
}

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gesture_studio=info".into()),
        )
        .init();

    info!("Starting Gesture Studio");

    let config = Arc::new(RwLock::new(AppConfig::load()));
    let (event_tx, event_rx) = crossbeam_channel::unbounded::<AppEvent>();

    let storage = Arc::new(Mutex::new(
        storage::Storage::open(&config.read().unwrap().storage_path()).unwrap_or_default(),
    ));

    let service = {
        let config_guard = config.read().unwrap();
        let mut svc = GestureService::new(&config_guard, event_tx.clone());
        svc.load_from_storage(&storage);
        svc
    };

    let service = Arc::new(Mutex::new(service));

    let app_state = AppState {
        config: config.clone(),
        event_tx: event_tx.clone(),
        storage: storage.clone(),
        service: service.clone(),
    };

    app::EventProcessor::spawn(event_rx, service.clone(), storage.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .setup(|app| {
            app::events_bridge::init(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app::commands::get_app_status,
            app::commands::get_config,
            app::commands::update_config,
            camera::commands::list_cameras,
            camera::commands::start_camera,
            camera::commands::stop_camera,
            gestures::commands::list_gestures,
            gestures::commands::create_gesture,
            gestures::commands::delete_gesture,
            actions::commands::list_actions,
            actions::commands::create_action,
            actions::commands::delete_action,
            actions::commands::execute_action,
            trainer::commands::start_training,
            trainer::commands::capture_training_sample,
            trainer::commands::finish_training,
            storage::commands::get_stats,
            vision::commands::submit_hand_frame,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Gesture Studio");
}
