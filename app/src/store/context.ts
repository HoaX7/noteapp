import type { Writable } from "svelte/store";
import storable from ".";
import type { ContextProps } from "../types";

const INITIAL_STATE = {} as ContextProps;

const ContextStore = {
	store: {} as Writable<ContextProps>,
	init() {
		if (typeof window === "undefined") return;
		const store = storable(INITIAL_STATE, "context-store", localStorage);
		if (store) {
			this.store = store;
		}
		return store; 
	},
	update(data: ContextProps) {
		this.store.set(data);
	},
	clear() {
		this.store.set(INITIAL_STATE);
	},
	getContext() {
		return this.store;
	}
};

export default ContextStore;