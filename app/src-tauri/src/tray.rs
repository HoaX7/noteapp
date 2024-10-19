use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Window};

use crate::shortcuts;

struct MenuItems{}
impl MenuItems {
    #[cfg(target_os = "windows")]
    const QUICKNOTES: (&str, &str) = ("quicknotes", "Quicknotes     Ctrl+Enter");
    #[cfg(target_os = "macos")]
    const QUICKNOTES: (&str, &str) = ("quicknotes", "Quicknotes     Cmd+Enter");
    const SETTINGS: (&str, &str) = ("settings", "Settings");
    const SHOW: (&str, &str) = ("show", "Show");
    const UPDATE: (&str, &str) = ("update", "Update");
    const QUIT: (&str, &str) = ("quit", "Quit");
}
const MAIN_WINDOW_LABEL: &str = "main";

struct TauriEvents{}
impl TauriEvents {
    const SHOW_SETTINGS: &str = "show_settings";
    const CHECK_UPDATE: &str = "check_update";
}

pub fn make_tray() -> SystemTray {
    let mut tray_menu = SystemTrayMenu::new();
    let menu_items = vec![MenuItems::QUICKNOTES, MenuItems::SETTINGS, MenuItems::SHOW];
    for item in menu_items {
        let custom_menu = CustomMenuItem::new(item.0, item.1);
        tray_menu = tray_menu.add_item(custom_menu);
    };
    let custom_menu = CustomMenuItem::new(MenuItems::UPDATE.0, MenuItems::UPDATE.1);
    tray_menu = tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(custom_menu);
    let custom_menu = CustomMenuItem::new(MenuItems::QUIT.0, MenuItems::QUIT.1);
    tray_menu = tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(custom_menu);
    let tray = SystemTray::new();
    tray.with_menu(tray_menu)
}

fn show_main_window(app: &AppHandle) -> Window {
    let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
    window
}

pub fn system_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick { .. } => {
            show_main_window(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                str if str == MenuItems::QUIT.0 => {
                    app.exit(0);
                }
                str if str == MenuItems::QUICKNOTES.0 => {
                    shortcuts::make_quicknote_window(&app.app_handle());
                }
                str if str == MenuItems::SETTINGS.0 => {
                    let window = show_main_window(app);
                    window.emit(TauriEvents::SHOW_SETTINGS, {}).unwrap();
                }
                str if str == MenuItems::SHOW.0 => {
                    show_main_window(app);
                }
                str if str == MenuItems::UPDATE.0 => {
                    let window = show_main_window(app);
                    window.emit(TauriEvents::CHECK_UPDATE, {}).unwrap();
                }
                &_ => {}
            }
        }
        _ => {}
    }
}