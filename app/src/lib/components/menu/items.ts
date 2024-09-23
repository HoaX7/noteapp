import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import { openShortNoteWindow } from "../../../api";
import { QUICKNOTE_WINDOW_LABEL, Shortcuts } from "../../../utils/constants";

export type MenuItemProps = {
  name: string;
  command: () => void;
  shortcut?: string;
  classname?: string;
  disabled?: boolean;
};
const emitSave = () => {
	const event = new KeyboardEvent("keydown", {
		bubbles: true,
		key: "s",
		ctrlKey: true,
		cancelable: true,
	});
	window.dispatchEvent(event);
};
const quitApp = () => {
	const quicknotesWindow = WebviewWindow.getByLabel(QUICKNOTE_WINDOW_LABEL);
	quicknotesWindow?.close?.();
	appWindow.close();
};
export const getMenuBar = (page?: string, os?: string, cb?: () => void) => {
	const shortcutObj = Shortcuts(os);
	return [
		{
			name: "File",
			items: [
				{
					name: "Save",
					command: emitSave,
					shortcut: shortcutObj.SAVE,
					disabled: !page,
				},
				{
					name: "quicknotes",
					command: openShortNoteWindow,
					shortcut: shortcutObj.QUICKNOTE,
				},
				{
					name: "Refresh",
					command: () => window.location.reload(),
					shortcut: "F5"
				},
				{
					name: "Settings",
					command: () => cb?.(),
				},
				{
					name: "Quit",
					command: quitApp,
					classname: "border-t",
				},
			],
		},
	];
};
