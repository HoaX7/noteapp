import type { Writable } from "svelte/store";
import storable from ".";
import type { SettingStoreProps } from "../types";

const INITIAL_STATE = {} as SettingStoreProps;

const SettingStore = {
	store: {} as Writable<SettingStoreProps>,
	init() {
		if (typeof window === "undefined") return;
		const store = storable(INITIAL_STATE, "settings-store", localStorage);
		if (store) {
			this.store = store;
		}
		return store; 
	},
	update(data: SettingStoreProps) {
		this.store.set(data || {});
	},
	clear() {
		this.store.set(INITIAL_STATE);
	},
	getContext() {
		return this.store;
	}
};

export default SettingStore;