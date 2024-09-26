<script lang="ts">
  import clsx from "clsx";
  import { createEventDispatcher } from "svelte";

  export let checked = false;
  $: console.log({ checked });
  
  const dispatch = createEventDispatcher();
</script>

<label class="relative inline-flex items-center cursor-pointer">
    <input type="checkbox" name="toggle" class="sr-only peer" on:change={() => dispatch("change")}>
    <div class={clsx("w-9 h-5 bg-gray-200 focus:outline-none rounded-full peer peer-checked:after:translate-x-full",
    "peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px]",
    "after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all",
    "peer-checked:bg-blue-600")}></div>
</label>
