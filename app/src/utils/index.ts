import { path } from "@tauri-apps/api";

export const debounce = (cb: any, timeout: number) => {
	let timer: any;
	return (...args: any[]) => {
		clearTimeout(timer);
		timer = setTimeout(() => {
			cb.apply(this, args);
		}, timeout);
	};
};

export const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const parseFilename = async (fn: string) => {
	const ext = await path.extname(fn);
	const filename = fn.split(`.${ext}`)[0];
	return {
		filename,
		ext
	};
};

export const getFullMonth = (month: number) => {
	return [
		"January",
		"February",
		"March",
		"April",
		"May",
		"June",
		"July",
		"August",
		"September",
		"October",
		"November",
		"December"
	][month];
};

export const parseFileModifiedDate = (date: Date) => {
	const today = new Date().getDate();
	if (today === date.getDate()) {
		return `Today ${date.toLocaleTimeString()}`;
	}
	return date.toLocaleDateString();
};