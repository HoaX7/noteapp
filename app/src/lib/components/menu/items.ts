import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import { openShortNoteWindow } from "../../../api";
import { SHORT_NOTE_WINDOW_LABEL } from "../../../utils/constants";

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
	const shortnotesWindow = WebviewWindow.getByLabel(SHORT_NOTE_WINDOW_LABEL);
	shortnotesWindow?.close?.();
	appWindow.close();
};
export const getMenuBar = (page?: string, cb?: () => void) => [
	{
		name: "File",
		items: [
			{
				name: "Save",
				command: emitSave,
				shortcut: "Ctrl+S",
				disabled: !page,
			},
			{
				name: "Shortnotes",
				command: openShortNoteWindow,
				shortcut: "Ctrl+Space",
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
