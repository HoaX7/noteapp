<script lang="ts">
  import { type Editor } from "@tiptap/core";
  import MenuButton from "./MenuButton.svelte";
  import Typography from "../common/Typography.svelte";
  import { createEventDispatcher } from "svelte";
  import { getList } from "./slashCommandList";

  export let editor: Editor;
  export let ref: null | HTMLDivElement = null;
  

  export let items = getList(editor);
  export let range: any;
  let activeIdx = 0;
  /**
   * This API is used to filter data
   * for suggestions.
   */
  export const search = (query: string) =>
    items.filter(({ name }) =>
      name.toLowerCase().startsWith(query.toLowerCase())
    );

  export const onKeyDown = (ev: KeyboardEvent, range: any) => {
    switch (ev.key) {
      case "ArrowUp":
        activeIdx = (activeIdx + items.length - 1) % items.length;
        return true;
      case "ArrowDown":
        activeIdx = (activeIdx + 1) % items.length;
        return true;
      case "Enter":
        if (ev.repeat) return false;
        triggerCommand(items[activeIdx], range);
        return true;
    }
    return false;
  };

  const dispatch = createEventDispatcher();

  const clearBlock = (range?: any) => {
    const selection = editor.state.selection;
    if (!range) {
      range = {
        from: selection.to - 1,
        to: selection.to
      }
    }
    editor
      .chain()
      .focus()
      .deleteRange(range)
      .run();
  };
  const triggerCommand = (item: { name: string; click: () => void }, range?: any) => {
    clearBlock(range);
    item?.click?.();
    /**
     * When an item is selected, hide the element.
     */
    dispatch("click");
  };
</script>

<div bind:this={ref} class="flex flex-col border shadow bg-white rounded-md">
  {#each items as item, idx}
    <MenuButton
      on:click={() => triggerCommand(item, range)}
      classname={idx === activeIdx ? "bg-gray-200" : ""}
    >
      <Typography variant="span" weight="normal" fontSize="sm">
        {item.name}
      </Typography>
    </MenuButton>
  {/each}
</div>
