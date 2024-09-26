<script lang="ts">
  import { type Editor } from "@tiptap/core";
  import DropDown from "../common/DropDown.svelte";
  import Typography from "../common/Typography.svelte";
  import Icon from "../common/Icon.svelte";
  import clsx from "clsx";
  import Toggle from "../common/Toggle.svelte";
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";
  import tableOptions from "./tableOptions";

  export let editor: Editor;
  const options = tableOptions(editor);
  let showOpts = false;

  const deleteTable = () => editor?.chain().focus().deleteTable().run();
</script>

<WindowEvent event="click" callback={() => showOpts = false} />
<div class="z-10 relative" on:click|stopPropagation role="none">
  <div class="bg-gray-100 rounded-md flex items-center">
      <button class="hover:bg-gray-200 p-2" on:click|stopPropagation={deleteTable}>
        <Icon src="assets/images/bin.svg" alt="delete-table" width="14" />
      </button>
      <button
        class="hover:bg-gray-200 px-2 p-1 border-l border-gray-300"
        on:click|stopPropagation={() => (showOpts = !showOpts)}
      >
        <Typography
          weight="normal"
          fontSize="base"
          variant="div"
          classname="flex gap-1"
        >
          Options <Icon
            src="assets/images/chevron.svg"
            alt="dd"
            width="14"
            class={clsx(
              showOpts ? "rotate-180" : "",
              "transition delay-150 duration-300 ease-in-out"
            )}
          />
        </Typography>
      </button>
  </div>
  {#if showOpts}
    <div class="absolute bg-gray-100 shadow rounded-b-md p-0.5 w-[200px]">
      {#each options as { name, key, icon, command }}
      <button class="w-full hover:bg-gray-200 rounded-md text-start px-2 py-1 justify-between flex"
        on:click|stopPropagation={() => {
            command();
        }}
      >
        <div class="flex gap-1 text-nowrap">
            <Icon alt={key} src={icon} width="14" />
            <Typography variant="div" fontSize="base" weight="normal" classname="">
                {name}
            </Typography>
        </div>
      </button>
      {/each}
    </div>
  {/if}
</div>
