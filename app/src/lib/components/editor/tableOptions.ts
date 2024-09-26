import { Editor } from "@tiptap/core";

export default (editor: Editor) => [
	{
		key: "header_row",
		name: "Header row",
		icon: "assets/images/table-header-row.svg",
		command: () => editor.chain().focus().toggleHeaderRow().run(),
	},
	{
		key: "header_column",
		name: "Header column",
		icon: "assets/images/table-header-column.svg",
		command: () => editor.chain().focus().toggleHeaderColumn().run(),
	},
	// {
	// 	key: "merge_cells",
	// 	name: "Merge cells",
	// 	icon: "assets/images/table-merge-cells.svg",
	// 	command: () => editor.chain().focus().mergeCells().run(),
	// },
	// {
	// 	key: "split_cells",
	// 	name: "Split cells",
	// 	icon: "assets/images/table-split-cells.svg",
	// 	command: () => editor.chain().focus().splitCell().run(),
	// },
	{
		key: "insert_row_before",
		name: "Add row before",
		icon: "assets/images/table-insert-row.svg",
		command: () => editor.chain().focus().addRowBefore().run()
	},
	{
		key: "insert_row_after",
		name: "Add row after",
		icon: "assets/images/table-insert-row.svg",
		command: () => editor.chain().focus().addRowAfter().run()
	},
	{
		key: "delete_row",
		name: "Delete row",
		icon: "assets/images/table-delete-row.svg",
		command: () => editor.chain().focus().deleteRow().run()
	},
	{
		key: "insert_column_before",
		name: "Add column before",
		icon: "assets/images/table-insert-col.svg",
		command: () => editor.chain().focus().addColumnBefore().run()
	},
	{
		key: "insert_column_after",
		name: "Add column after",
		icon: "assets/images/table-insert-col.svg",
		command: () => editor.chain().focus().addColumnAfter().run()
	},
	{
		key: "delete_column",
		name: "Delete column",
		icon: "assets/images/table-delete-col.svg",
		command: () => editor.chain().focus().deleteColumn().run()
	}
];