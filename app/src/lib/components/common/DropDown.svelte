<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Typography from "./Typography.svelte";
  import Icon from "./Icon.svelte";
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";

    type IItem = {
        key: string;
        name: string;
    };
    export let items: IItem[] = [];
    export let selectedItem = {} as IItem;
    const dispatch = createEventDispatcher();

    let showdd = false;
</script>

<!-- <WindowEvent event="mouseup" callback={() => showdd = false} /> -->
<div class="rounded border" on:click|stopPropagation role="none">
    <Typography variant="div" weight="normal" fontSize="base" classname="p-1 px-2 flex justify-between items-center cursor-pointer"
        on:click={() => showdd = !showdd}
    >
        {selectedItem.name || "Select"} <Icon src="/assets/images/chevron.svg" width="18" alt="dd" class={showdd ? "rotate-180" : ""} />
    </Typography>
    {#if showdd}
        <hr class="mx-3" />
        <div class="mt-2">
            {#if items.length === 0}
        <Typography variant="div" weight="normal" fontSize="sm" classname="p-1 px-2 text-gray-600">
            No Items
        </Typography>
        {:else}
        {#each items as item}
        <button class="w-full text-start p-1 px-2 hover:bg-gray-100 disabled:text-gray-600"
            on:click={() => {
                selectedItem = item;
                dispatch("select", item);
            }}
            disabled={selectedItem.key === item.key}
        >
            <Typography variant="div" weight="normal" fontSize="base">
                {item.name}
            </Typography>
        </button>
        {/each}
        {/if}
        </div>
    {/if}
</div>
