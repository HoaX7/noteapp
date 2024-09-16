import { appWindow } from "@tauri-apps/api/window";
import { openShortNoteWindow } from "../../../api";

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
				name: "Settings",
				command: () => cb?.(),
			},
			{
				name: "Quit",
				command: appWindow.close,
				classname: "border-t",
			},
		],
	},
];
