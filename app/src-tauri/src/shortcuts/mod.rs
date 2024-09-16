use tauri::{App, AppHandle, GlobalShortcutManager, Manager, WindowBuilder, WindowUrl};
use window_shadows::set_shadow;

struct ShortNotesWindow;
impl ShortNotesWindow {
    const LABEL: &str = "short_notes_window";
    const SHORTCUT: &str = "CommandOrControl+Space";
    const W: f64 = 400.0;
    const H: f64 = 200.0;
    const URL: &str = "shortNotes";
}

pub fn register_shortcuts(app: &mut App) {
    let handle = app.handle();
    // Enables the user to take quick notes anytime.
    app.global_shortcut_manager().register(ShortNotesWindow::SHORTCUT, move || {
        make_shortnotes_window(&handle);
    }).unwrap();
}

pub fn make_shortnotes_window(handle: &AppHandle) {
    if let Some(window) = handle.get_window(ShortNotesWindow::LABEL) {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let window = WindowBuilder::new(handle, ShortNotesWindow::LABEL, WindowUrl::App(ShortNotesWindow::URL.into()))
            .resizable(false)
            .inner_size(ShortNotesWindow::W, ShortNotesWindow::H)
            .minimizable(false)
            .decorations(false)
            .always_on_top(true)
            .build()
            .unwrap();

        set_shadow(&window, true).expect("window shadow unsupported for this platform");
        window.show().unwrap();
    }
}
