import type { NodeViewRendererProps } from "@tiptap/core";
import type { NodeView } from "@tiptap/pm/view";
import TableOptions from "../TableOptions.svelte";
import SvelteRenderer from "./renderer";
import tippy, { type Instance, type Props } from "tippy.js";

export default ({ editor, node }: NodeViewRendererProps): NodeView => {
	const dom = document.createElement("div");
	dom.classList.add("tableWrapper");
	const table = document.createElement("table");
	table.setAttribute("tabindex", "-1");
	let renderer: SvelteRenderer | null = null;
	let tippyEl: Instance<Props>;
	let wrapper = document.createElement("div");
	const component = new TableOptions({
		target: wrapper,
		props: { editor }
	});
	renderer = new SvelteRenderer(component, { element: wrapper });
	tippyEl = tippy(editor.options.element, {
		interactive: true,
		content: renderer.dom,
		getReferenceClientRect: () => {
			const rect = table.getBoundingClientRect();
			return rect;
		},
		trigger: "manual",
		showOnCreate: false,
		placement: "top-start"
	});
	dom.appendChild(table);
	table.addEventListener("focus", () => {
		tippyEl?.show();
	});
	return {
		dom,
		contentDOM: table,
		destroy() {
			tippyEl?.destroy();
			renderer?.destroy();
			dom.remove();
			wrapper?.remove();
		},
	};
};