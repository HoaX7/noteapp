<script>
  import CustomWindow from "$lib/components/customWindow/Windows.svelte";
  import Sidebar from "$lib/components/sidebar/Index.svelte";
  import { onMount } from "svelte";
  import "../app.css";
  import ContextStore from "../store/context";
  import { platform } from "@tauri-apps/api/os";
  import MacOs from "$lib/components/customWindow/MacOs.svelte";
  import { page } from "$app/stores";

  ContextStore.init();
  let os = ""; // darwin | linux | win32. see https://tauri.app/v1/api/js/os for all platforms.
  onMount(async () => {
    os = await platform();
  });

</script>

{#if $page.url.pathname === "/shortNotes"}
  <slot />
{:else}
  {#if os === "darwin"}
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
