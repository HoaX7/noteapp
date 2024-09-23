<script>
  import CustomWindow from "$lib/components/customWindow/Windows.svelte";
  import Sidebar from "$lib/components/sidebar/Index.svelte";
  import { onMount } from "svelte";
  import "../app.css";
  import ContextStore from "../store/context";
  import { platform } from "@tauri-apps/api/os";
  import MacOs from "$lib/components/customWindow/MacOs.svelte";
  import { page } from "$app/stores";
  import SettingStore from "../store/settings";
  import { MAC_OS, TAURI_EVENTS } from "../utils/constants";
  // import "../api/update";
  import { getVersion } from "@tauri-apps/api/app";
  import TauriEventListener from "$lib/hooks/TauriEventListener.svelte";
  import Settings from "$lib/components/settings/Index.svelte";

  ContextStore.init();
  const ctx = SettingStore.init();
  let showSettings = false;
  onMount(async () => {
    const [os, version] = await Promise.all([
      platform(),
      getVersion()
    ]); // darwin | linux | win32. see https://tauri.app/v1/api/js/os for all platforms.
    ctx?.update((store) => {
      store.os = os;
      store.version = version;
      return store;
    })
  });

  const closeSettings = () => showSettings = false;
</script>

<TauriEventListener eventName={TAURI_EVENTS.SHOW_SETTINGS} callback={() => showSettings = true} />
  {#if showSettings}
    <Settings on:close={closeSettings} />
  {/if}
{#if $page.url.pathname === "/quickNotes"}
  <slot />
{:else}
  {#if $ctx?.os === MAC_OS}
    <MacOs />
  {:else}
    <CustomWindow />
  {/if}
  <div class="grid grid-cols-8 h-full pt-8">
    <div class="col-span-2"><Sidebar /></div>
    <div class="p-3 overflow-auto col-span-6">
      <slot />
    </div>
  </div>
{/if}
