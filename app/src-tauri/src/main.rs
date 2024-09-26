// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;
use tray::*;
mod commands;
mod shortcuts;
use commands::*;
mod controller;
use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .system_tray(make_tray())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            {
                let cloned_window = window.clone();
                window.on_window_event(move |event| match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        cloned_window.hide().unwrap();
                        api.prevent_close();
                    }
                    _ => {}
                });
            }

            set_shadow(&window, true).expect("window shadow unsupported for this platform");
            shortcuts::register_shortcuts(app);
            Ok(())
        })
        .on_system_tray_event(system_tray_events)
        .invoke_handler(tauri::generate_handler![
            save_content,
            get_content,
            get_file_list,
            rename_file,
            delete_file,
            open_shortnote_window,
            get_settings,
            save_settings,
            search_contents
        ])
        .run(tauri::generate_context!())
        .expect("error running application");
}
