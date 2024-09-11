import { Node } from "@tiptap/core";
import { PluginKey } from "@tiptap/pm/state";
import Suggestion from "@tiptap/suggestion";
import type { Instance, Props } from "tippy.js";
import tippy from "tippy.js";
import SvelteRenderer from "../nodeView/renderer";
import SlashCommandMenu from "../SlashCommandMenu.svelte";
import type { SvelteComponent } from "svelte";

type SlashCommandOptions = {
	component?: SvelteComponent | null;
	tippy?: Instance<Props>;
};
const EXT_NAME = "SlashCommands";
const CHAR = "/";
export const SlashCommandPluginKey = new PluginKey(EXT_NAME);
export const SlashCommands = Node.create<SlashCommandOptions>({
	name: EXT_NAME,
	selectable: false,
	group: "inline",
	inline: true,
	atom: true,
	addOptions() {
		return { component: null };
	},
	addProseMirrorPlugins() {
		const { element: editorElement } = this.editor.options;
		return [
			Suggestion({
				editor: this.editor,
				char: CHAR,
				pluginKey: SlashCommandPluginKey,
				allowSpaces: false,
				/**
				 * We only show the suggestion if ('/') is typed.
				 */
				allow: ({ state }) => {
					const node = state.selection.$from.node();
					if (!node) return false;
					return node.textBetween(0, 1) === CHAR;
				},
				items: ({ query }) => {
					return this.options.component?.search?.(query) || [];
				},
				render: () => {
					let renderer: SvelteRenderer;
					let wrapper: HTMLDivElement;
					return {
						onStart: () => {
							wrapper = document.createElement("div");
							this.options.component = new SlashCommandMenu({
								target: wrapper,
								props: { editor: this.editor, } as any
							});
							renderer = new SvelteRenderer(this.options.component, { element: wrapper });
							this.options.tippy = tippy(editorElement, {
								interactive: true,
								content: renderer.dom,
								trigger: "manual",
								placement: "bottom-start",
								getReferenceClientRect: null,
								showOnCreate: true,
							});
						},
						onKeyDown: (props) => {
							if (props.event.key === "Escape") {
								this.options.tippy?.hide();
								return true;
							}
							return this.options.component?.onKeyDown?.(props.event);;
						},
						onUpdate: (props) => {
							renderer.updateProps(props);
						},
						onExit: () => {
							this.options.tippy?.destroy();
							renderer?.destroy();
							wrapper?.remove();
						},
					};
				},
			}),
		];
	},
});
