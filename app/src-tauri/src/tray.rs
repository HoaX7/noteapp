use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use crate::shortcuts;

struct MenuItems{}
impl MenuItems {
    const SHORTNOTES: (&str, &str) = ("shortnotes", "Shortnotes     Ctrl+Space");
    const SETTINGS: (&str, &str) = ("settings", "Settings");
    const QUIT: (&str, &str) = ("quit", "Quit");
}
const MAIN_WINDOW_LABEL: &str = "main";

pub fn make_tray() -> SystemTray {
    let mut tray_menu = SystemTrayMenu::new();
    let menu_items = vec![MenuItems::SHORTNOTES, MenuItems::SETTINGS];
    for item in menu_items {
        let custom_menu = CustomMenuItem::new(item.0, item.1);
        tray_menu = tray_menu.add_item(custom_menu);
    };
    let custom_menu = CustomMenuItem::new(MenuItems::QUIT.0, MenuItems::QUIT.1);
    tray_menu = tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(custom_menu);
    let tray = SystemTray::new();
    tray.with_menu(tray_menu)
}

pub fn system_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick { .. } => {
            let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
            window.show().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                str if str == MenuItems::QUIT.0 => {
                    app.exit(0);
                }
                str if str == MenuItems::SHORTNOTES.0 => {
                    shortcuts::make_shortnotes_window(&app.app_handle());
                }
                str if str == MenuItems::SETTINGS.0 => {
                    let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
                    window.show().unwrap();
                    window.emit("show_settings", {}).unwrap();
                }
                &_ => {}
            }
        }
        _ => {}
    }
}