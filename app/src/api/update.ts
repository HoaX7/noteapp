// import {
// 	checkUpdate,
// 	installUpdate,
// 	onUpdaterEvent,
// } from "@tauri-apps/api/updater";
// import { relaunch } from "@tauri-apps/api/process";

import { relaunch } from "@tauri-apps/api/process";
import { checkUpdate, installUpdate, type UpdateManifest } from "@tauri-apps/api/updater";
export { checkUpdate };
// const unlisten = await onUpdaterEvent(({ error, status }) => {
// 	// This will log all updater events, including status updates and errors.
// 	console.log("Updater event", error, status);
// });
// // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
// unlisten();

export const installUpdates = async (manifest: UpdateManifest) => {
	console.log(
		`Installing update ${manifest.version}, ${manifest.date}, ${manifest.body}`
	);
	// Install the update. This will also restart the app on Windows!
	await installUpdate();

	// On macOS and Linux you will need to restart the app manually.
	// You could use this step to display another confirmation dialog.
	await relaunch();
};