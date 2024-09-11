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
