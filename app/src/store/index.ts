import { writable, get, type Writable } from "svelte/store";

export default function storable<T>(data: T, key = "store", storage = localStorage): Writable<T> | undefined {
	if (typeof window === "undefined") {
		console.error("Store can only be initialized on the client browser. Did you mean to call storable onMount?");
		return;
	}
	const store = writable<T>(data);
	storage[key] && store.set(JSON.parse(storage[key]));

	return {
		subscribe: store.subscribe,
		set: (n: T) => {
			storage[key] = JSON.stringify(n);
			store.set(n);
		},
		/**
         * Use this method to get the updated data in callback
         * @param cb
         */
		update: (cb: (data: T) => T) => {
			const updatedStore = cb(get(store));
			storage[key] = JSON.stringify(updatedStore);
			store.set(updatedStore);
		}
	};
}