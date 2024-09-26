<script lang="ts">
  import clsx from "clsx";
  import Typography from "./Typography.svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  export let empty = false;
  export let containerClass = "";
</script>

<div
  class={clsx("fixed inset-0 z-[999] grid h-screen w-screen bg-black bg-opacity-60 backdrop-blur-sm", containerClass ? containerClass : "place-items-center")}>
  {#if empty}
  <div class="relative bg-white rounded w-3/4">
    <slot />
  </div>
  {:else}
  <div
    class="relative bg-white w-3/4 h-3/4 rounded">
    <Typography variant="div" fontSize="2xl" weight="bold" 
        classname={clsx("flex justify-between  items-center p-4 font-medium text-slate-800 border-b")}>
        <slot name="title" /> 
        <button class="mx-2" on:click={() => dispatch("close")}>
            &times;
        </button>
    </Typography>
    <div class="relative p-4 overflow-auto h-full">
      <slot />
    </div>
  </div>
  {/if}
</div>
