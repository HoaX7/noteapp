// import {
// 	checkUpdate,
// 	installUpdate,
// 	onUpdaterEvent,
// } from "@tauri-apps/api/updater";
// import { relaunch } from "@tauri-apps/api/process";
  
// const unlisten = await onUpdaterEvent(({ error, status }) => {
// 	// This will log all updater events, including status updates and errors.
// 	console.log("Updater event", error, status);
// });

// try {
// 	const { shouldUpdate, manifest } = await checkUpdate();
// 	console.log({
// 		shouldUpdate,
// 		manifest 
// 	});
// 	if (shouldUpdate) {
// 		const ask = window.confirm("A new update is available");
// 		// You could show a dialog asking the user if they want to install the update here.
// 		console.log(
// 			`Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
// 		);
  
// 		// Install the update. This will also restart the app on Windows!
// 		// await installUpdate();
  
// 		// // On macOS and Linux you will need to restart the app manually.
// 		// // You could use this step to display another confirmation dialog.
// 		// await relaunch();
// 	}
// } catch (error) {
// 	console.error(error);
// }
  
// // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
// unlisten();
