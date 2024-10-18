export type ContextProps = {
    page: string;
    ext: string;
    isShortNote?: boolean;
    modified: Date;
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

export type FileListProps = {
    filename: string;
    modified: number;
}