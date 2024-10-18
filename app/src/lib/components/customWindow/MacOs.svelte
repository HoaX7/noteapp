<script lang="ts">
  import clsx from "clsx";
  import Updater from "../settings/Updater.svelte";

  export let updateManifest;
</script>

<div data-tauri-drag-region class={clsx(
    "z-[999] h-8 flex bg-gray-200 fixed user-select-none top-0 left-0 right-0 bg-gradient",
    "flex justify-end items-center"
  )}>
  <div class="mx-2"><Updater {updateManifest} on:updatecomplete /></div>
</div>
