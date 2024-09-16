<script lang="ts">
  import clsx from "clsx";
  import Typography from "./Typography.svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
</script>

<div
  class="fixed inset-0 z-[999] grid h-screen w-screen place-items-center bg-black bg-opacity-60 backdrop-blur-sm"
>
  <div
    class="relative bg-white w-3/4 h-3/4 rounded"
  >
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
</div>
