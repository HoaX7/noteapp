export type ContextProps = {
    page: string;
    ext: string;
    isShortNote?: boolean;
    theme?: "dark" | "light";
    forceRefresh?: boolean;
}

export type SettingProps = {
    save_files_to_dir: string;
    notion?: boolean;
}
