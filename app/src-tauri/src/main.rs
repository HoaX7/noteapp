// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;
use std::io::{Read, Write};
use log::error;
use tauri_plugin_log::LogTarget;

use noteapp_lib::{app_settings, fs};
use tray::*;
mod commands;
mod shortcuts;
use commands::*;
mod controller;
use tauri::{AppHandle, Manager};
use window_shadows::set_shadow;

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };
        match info.location() {
            Some(location) => {
                error!(
                    target: "scribe::panic", "App panicked '{}': {}:{}",
                    msg,
                    location.file(),
                    location.line(),
                );
            }
            None => error!(
                target: "scribe::panic",
                "App panicked at '{}'",
                msg,
            ),
        }
    }));
    tauri::Builder::default()
        .system_tray(make_tray())
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
        ]).build())
        .setup(|app| {
            let handle = app.handle();
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

                // add guide to the default path
                save_guide(&handle);
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

fn save_guide(handle: &AppHandle) {
    let resource_path = handle.path_resolver()
    .resolve_resource("./docs/guide_DO_NOT_EDIT.md")
    .expect("failed to resolve resource");
    let mut file = std::fs::File::open(&resource_path).unwrap();
    let contents = &mut String::default();
    file.read_to_string(contents).unwrap();
    let path = app_settings::make_path("guide_DO_NOT_EDIT.md");
    fs::create_dir_all(path.as_path()).unwrap();
    fs::try_write(path.as_path(), contents.as_bytes(), false).unwrap();
    file.flush().unwrap();
}
