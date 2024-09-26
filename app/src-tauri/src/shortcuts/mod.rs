use tauri::{App, AppHandle, GlobalShortcutManager, Manager, WindowBuilder, WindowUrl};
use window_shadows::set_shadow;
mod disable_quicknote;

struct QuickNoteWindow;
impl QuickNoteWindow {
    const LABEL: &str = "quicknotes_window";
    const SHORTCUT: &str = "CommandOrControl+Enter";
    const W: f64 = 400.0;
    const H: f64 = 200.0;
    const URL: &str = "quickNotes";
}

pub fn register_shortcuts(app: &mut App) {
    let handle = app.handle();
    // Enables the user to take quick notes anytime.
    app.global_shortcut_manager().register(QuickNoteWindow::SHORTCUT, move || {
        make_quicknote_window(&handle);
    }).unwrap();
}

pub fn make_quicknote_window(handle: &AppHandle) {
    if disable_quicknote::deny_quicknote() {
        return;
    }
    if let Some(window) = handle.get_window(QuickNoteWindow::LABEL) {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let window = WindowBuilder::new(handle, QuickNoteWindow::LABEL, WindowUrl::App(QuickNoteWindow::URL.into()))
            .resizable(false)
            .inner_size(QuickNoteWindow::W, QuickNoteWindow::H)
            .minimizable(false)
            .decorations(false)
            .always_on_top(true)
            .build()
            .unwrap();

        set_shadow(&window, true).expect("window shadow unsupported for this platform");
        window.show().unwrap();
    }
}
