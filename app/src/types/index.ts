export type ContextProps = {
    page: string;
    ext: string;
    isShortNote?: boolean;
}

export type SettingProps = {
    save_files_to_dir: string;
    notion?: boolean;
}

export type SettingStoreProps = {
    theme?: "dark" | "light";
    lastQuickNoteAt?: number;
    os?: string;
    version?: string;
}