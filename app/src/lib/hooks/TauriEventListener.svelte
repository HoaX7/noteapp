<script lang="ts">
  import {
    listen,
    type EventCallback,
    type UnlistenFn,
  } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let unlisten: UnlistenFn;
  export let eventName: string;
  export let callback: EventCallback<any>;

  onMount(async () => {
    unlisten = await listen(eventName, callback);
  });

  onDestroy(() => {
    unlisten?.();
  });
</script>
