// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app_state;
pub mod core;
pub mod screencapture;
pub mod system_tray;

use app_state::{app_state::AppState, app_state_factory};
use core::tauri_command::{
    find_configuration, get_app_directory_path, get_inference_server_url, is_server_up,
    selected_profiles, upsert_configuration,
};
use log::{info, LevelFilter};
use screencapture::tauri_command::{assert_screen_capture_permissions, capture_screen};
use system_tray::{build_menu, on_system_tray_event};
use tauri::{AppHandle, Manager, RunEvent, State, SystemTray, WindowEvent};
use tauri_plugin_log::LogTarget;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

#[tokio::main]
async fn main() {
    let log_builder = tauri_plugin_log::Builder::default()
        .targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Stderr,
            LogTarget::Webview,
        ])
        .level(LevelFilter::Warn);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(build_menu()))
        .on_system_tray_event(on_system_tray_event)
        .plugin(log_builder.build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(setup)
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![
            is_server_up,
            capture_screen,
            assert_screen_capture_permissions,
            upsert_configuration,
            find_configuration,
            get_inference_server_url,
            get_app_directory_path,
            selected_profiles,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(run);
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let app_handle = app.handle();
    tauri::async_runtime::spawn(async move {
        match app_state_factory::create_and_bind(&app_handle).await {
            Ok(_) => {}
            Err(e) => panic!("Failed to create app state: {}", e),
        }

        let state: State<'_, AppState> = app_handle.state();
        state.inference_server.serve().await;
    });

    Ok(())
}

fn run(_app_handle: &AppHandle, event: RunEvent) -> () {
    match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    }
}

fn on_window_event(event: tauri::GlobalWindowEvent) -> () {
    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            info!("Window CloseRequested");
            #[cfg(not(target_os = "macos"))]
            {
                event.window().hide().unwrap();
            }

            #[cfg(target_os = "macos")]
            {
                tauri::AppHandle::hide(&event.window().app_handle()).unwrap();
            }
            api.prevent_close();
        }
        WindowEvent::Destroyed => {
            info!("Window destroyed");

            let app_handle: tauri::AppHandle = event.window().app_handle();
            app_handle.save_window_state(StateFlags::all()).ok();
        }
        _ => {}
    };
}
