export enum TAURI_EVENTS {
    REFRESH_NOTES = "refresh_notes",
    SHOW_SETTINGS = "show_settings",
    CLOSE_SETTINGS = "close_settings",
    RELOAD_FILES = "reload_files",
    PAGE_MODIFIED = "page_modified"
}

export const QUICKNOTE_WINDOW_LABEL = "quicknotes_window";

export const Shortcuts = (os?: string) => ({
	QUIT: `${os === MAC_OS ? "Cmd" : "Ctrl"}+Q`,
	SAVE: `${os === MAC_OS ? "Cmd" : "Ctrl"}+S`,
	QUICKNOTE: `${os === MAC_OS ? "Cmd" : "Ctrl"}+Enter`
});

export const MAC_OS = "darwin";