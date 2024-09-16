import type { Editor } from "@tiptap/core";
import type { Level } from "@tiptap/extension-heading";

const headerLevels: Level[] = [ 1, 2, 3, 4, 5, 6 ];
export const getList = (editor: Editor) => [
	{
		name: "Paragraph",
		click: () =>
			editor.chain().focus().toggleNode("paragraph", "paragraph").run(),
	},
	...headerLevels.map((level) => ({
		name: `Heading ${level}`,
		click: () => editor.chain().focus().toggleHeading({ level }).run(),
	})),
	{
		name: "Bullet List",
		click: () => editor.chain().focus().toggleBulletList().run(),
	},
	{
		name: "Todo List",
		click: () => editor.chain().focus().toggleTaskList().run(),
	},
	{
		name: "Divider",
		click: () =>
			editor.chain().focus().setHorizontalRule().run(),
	},
	{
		name: "Code Block",
		click: () => editor.chain().focus().toggleCodeBlock().run(),
	},
	{
		name: "Ordered List",
		click: () => editor.chain().focus().toggleOrderedList().run(),
	},
	{
		name: "Block Quotes",
		click: () => editor.chain().focus().toggleBlockquote().run(),
	},
];