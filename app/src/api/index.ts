import { invoke } from "@tauri-apps/api/tauri";
import type { FileListProps, SettingProps } from "../types";
import { isEmptyObject } from "@tiptap/core";

/**
 * Note: When calling invoke it is necessary to use the same
 * command and parameter name that matches the
 * tauri command on the backend.
 * The args passed need to be camelcase on the frontend
 * and snake case on the backend.
 */

type P = { path: string; append?: boolean; text: string; };
export const saveContent = async ({ path, append = false, text }: P) => {
	return invoke("save_content", {
		path,
		text,
		append
	});
};

export const getContent = async (path: string): Promise<string> => {
	return invoke("get_content", { path });
};

export const getFileList = async (): Promise<FileListProps[]> => {
	return invoke("get_file_list", {});
};

export const deleteFile = async (path: string) => {
	return invoke("delete_file", { path });
};

export const renameFile = async (curName: string, newName: string) => {
	return invoke("rename_file", {
		curName,
		newName
	});
};

export const openShortNoteWindow = () => {
	return invoke("open_shortnote_window", {});
};

export const getSettings = async (): Promise<SettingProps> => {
	return invoke("get_settings", {});
};

export const saveSettings = async (newSettings: Partial<SettingProps>) => {
	if (isEmptyObject(newSettings)) return;
	return invoke("save_settings", { newSettings });
};

export const searchContent = async (query: string): Promise<string[]> => {
	if (!query) return [];
	return invoke("search_contents", { query });
};