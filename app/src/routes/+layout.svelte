<script lang="ts">
  import CustomWindow from "$lib/components/customWindow/Windows.svelte";
  import Sidebar from "$lib/components/sidebar/Index.svelte";
  import { onDestroy, onMount } from "svelte";
  import "../app.css";
  import ContextStore from "../store/context";
  import { platform } from "@tauri-apps/api/os";
  import MacOs from "$lib/components/customWindow/MacOs.svelte";
  import { page } from "$app/stores";
  import SettingStore from "../store/settings";
  import { MAC_OS, TAURI_EVENTS } from "../utils/constants";
  import { getVersion } from "@tauri-apps/api/app";
  import TauriEventListener from "$lib/hooks/TauriEventListener.svelte";
  import Settings from "$lib/components/settings/Index.svelte";
  import { checkUpdate } from "../api/update";

  ContextStore.init();
  const ctx = SettingStore.init();
  let showSettings = false;
  let timer: any;
  let updateManifest = {
    updateAvailable: false,
    manifest: {}
  };
  onMount(async () => {
    const [os, version] = await Promise.all([
      platform(),
      getVersion()
    ]); // darwin | linux | win32. see https://tauri.app/v1/api/js/os for all platforms.
    ctx?.update((store) => {
      store.os = os;
      store.version = version;
      return store;
    });
    pollupdate();
  });

  const closeSettings = () => showSettings = false;

  const pollupdate = () => {
    timer = setInterval(async () => {
      if (updateManifest.updateAvailable) {
        clearInterval(timer);
        return;
      }
      // check for updates.
      try {
        const { shouldUpdate, manifest } = await checkUpdate();
        if (!shouldUpdate || !manifest) return;
        updateManifest.updateAvailable = true;
        updateManifest.manifest = manifest;
      } catch (err) {
        console.error("err", err)
      }
    }, 60 * 60 * 1000 * 6);
  }
  const handleUpdateComplete = () => {
    updateManifest = {
      updateAvailable: false,
      manifest: {}
    };
    pollupdate();
  }

  onDestroy(() => {
    clearInterval(timer);
  })
</script>

<TauriEventListener eventName={TAURI_EVENTS.SHOW_SETTINGS} callback={() => showSettings = true} />
  {#if showSettings}
    <Settings on:close={closeSettings} />
  {/if}
{#if $page.url.pathname === "/quickNotes"}
  <slot />
{:else}
  {#if $ctx?.os === MAC_OS}
    <MacOs updateManifest={updateManifest} on:updatecomplete={handleUpdateComplete} />
  {:else}
    <CustomWindow updateManifest={updateManifest} on:updatecomplete={handleUpdateComplete} />
  {/if}
  <div class="grid grid-cols-8 h-full pt-8">
    <div class="col-span-2"><Sidebar /></div>
    <div class="p-3 overflow-auto col-span-6">
      <slot />
    </div>
  </div>
{/if}
