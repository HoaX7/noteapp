<script lang="ts">
  import clsx from "clsx";
  import Typography from "../common/Typography.svelte";
  import type { MenuItemProps } from "./items";
  import { createEventDispatcher } from "svelte";

  export let items: MenuItemProps[];
  export let show: boolean;
  export let containerClass =
    "absolute border bg-white shadow-lg rounded-md w-56";
  export let selected = "";

  const dispatch = createEventDispatcher();
</script>

<div class={clsx(show ? "" : "hidden", containerClass)}>
  {#each items as item}
    <div class={item.classname}>
      <div
        class={clsx(
          "rounded-md m-0.5 px-5 py-1",
          "flex justify-between",
          selected.toLowerCase() === item.name.toLowerCase()
            ? "bg-gray-200"
            : "",
          item.disabled
            ? "opacity-60 cursor-default"
            : "cursor-pointer hover:bg-gray-200"
        )}
        role="button"
        tabindex="-1"
        on:click|stopPropagation={() => {
          !item.disabled && item.command();
          dispatch("close");
        }}
        aria-disabled={item.disabled}
        on:keypress
      >
        <Typography variant="div" weight="normal" fontSize="sm">
          {item.name}
        </Typography>
        {#if item.shortcut}
          <Typography variant="div" weight="normal" fontSize="sm">
            {item.shortcut}
          </Typography>
        {/if}
      </div>
    </div>
  {/each}
</div>
