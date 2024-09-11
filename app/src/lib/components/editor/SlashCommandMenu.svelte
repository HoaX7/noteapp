<script lang="ts">
  import { type Editor } from "@tiptap/core";
  import type { Level } from "@tiptap/extension-heading";
  import MenuButton from "./MenuButton.svelte";
  import Typography from "../common/Typography.svelte";
  import { createEventDispatcher, onMount } from "svelte";

  export let editor: Editor;
  export let ref: null | HTMLDivElement = null;
  const headerLevels: Level[] = [1, 2, 3, 4, 5, 6];
  const menuItems = [
    ...headerLevels.map((level) => ({
      name: `Heading ${level}`,
      click: () => editor.chain().focus().toggleHeading({ level }).run(),
    })),
    {
      name: "Paragraph",
      click: () =>
        editor.chain().focus().toggleNode("paragraph", "paragraph").run(),
    },
    {
      name: "Bullet List",
      click: () => editor.chain().focus().toggleBulletList().run(),
    },
    {
      name: "Check List",
      click: () => editor.chain().focus().toggleTaskList().run(),
    },
    {
      name: "Ordered List",
      click: () => editor.chain().focus().toggleOrderedList().run(),
    },
    {
      name: "Code Block",
      click: () => editor.chain().focus().toggleCodeBlock().run(),
    },
    {
      name: "Block Quotes",
      click: () => editor.chain().focus().toggleBlockquote().run(),
    },
  ];

  export let items = menuItems;
  let activeIdx = 0;
  /**
   * This API is used to filter data
   * for suggestions.
   */
  export const search = (query: string) =>
    items.filter(({ name }) =>
      name.toLowerCase().startsWith(query.toLowerCase())
    );

  export const onKeyDown = (ev: KeyboardEvent) => {
    if (ev.repeat) return;
    switch (ev.key) {
      case "ArrowUp":
        activeIdx = (activeIdx + items.length - 1) % items.length;
        return true;
      case "ArrowDown":
        activeIdx = (activeIdx + 1) % items.length;
        return true;
      case "Enter":
        triggerCommand(items[activeIdx]);
        return true;
    }
    return false;
  };

  const dispatch = createEventDispatcher();

  const clearBlock = () => {
    const selection = editor.state.selection;
    editor
      .chain()
      .focus()
      .insertContentAt(
        {
          from: selection.from - 1,
          to: selection.to,
        },
        ""
      )
      .run();
  };
  const triggerCommand = (item: { name: string; click: () => void }) => {
    item.click();
    /**
     * When an item is selected, hide the element.
     */
    dispatch("click");
    clearBlock();
  };
</script>

<div bind:this={ref} class="flex flex-col border shadow bg-white rounded-md">
  {#each items as item, idx}
    <MenuButton
      on:click={() => triggerCommand(item)}
      classname={idx === activeIdx ? "bg-gray-200" : ""}
    >
      <Typography variant="span" weight="normal" fontSize="sm">
        {item.name}
      </Typography>
    </MenuButton>
  {/each}
</div>
