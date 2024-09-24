import type { NodeViewRendererProps } from "@tiptap/core";
import lowlight, { registeredLangs } from "../extensions/registerCodeExt";

export default ({
	getPos,
	editor,
	node: { attrs: { language: defaultLanguage } },
}: NodeViewRendererProps) => {
	const dom = document.createElement("div");
	dom.classList.add("code-block");

	const select = document.createElement("select");
	select.classList.add("bg-gray-100", "px-2", "rounded", "py-1");
	select.contentEditable = "false";
	select.onchange = async (e: any) => {
		if (typeof getPos !== "function") return;
		const lang = e.target.value;
		editor.view.dispatch(
			editor.view.state.tr.setNodeMarkup(getPos(), undefined, { language: lang })
		);
	};
	const optionAuto = document.createElement("option");
	optionAuto.innerHTML = "auto";
	select.appendChild(optionAuto);

	registeredLangs.map((lang: string, idx: number) => {
		const option = document.createElement("option");
		option.setAttribute("key", `${idx}`);
		option.setAttribute("value", lang);
		if (lang === defaultLanguage) {
			option.setAttribute("selected", "true");
		}
		option.innerHTML = lang;
		select.appendChild(option);
	});

	const pre = document.createElement("pre");
	const content = document.createElement("code");
	pre.appendChild(content);
	dom.append(select, pre);

	return {
		dom,
		contentDOM: content
	};
};
