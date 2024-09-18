<script lang="ts">
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";
  import ContextStore from "../../../store/context";
  import Icon from "../common/Icon.svelte";
  import Typography from "../common/Typography.svelte";
  import MenuItems from "./MenuItems.svelte";
  import { getMenuBar } from "./items";
  import Settings from "../settings/Index.svelte";
  import TauriEventListener from "$lib/hooks/TauriEventListener.svelte";
  import { TAURI_EVENTS } from "../../../utils/constants";
  import { emit } from "@tauri-apps/api/event";

  const ctx = ContextStore.getContext();
  let menubar = [];
  let activeMenu = "";
  let showSettings = false;

  const closeMenu = () => {
    activeMenu = "";
  };
  $: menubar = getMenuBar($ctx.page, () => (showSettings = true));

  const closeSettings = () => {
    showSettings = false;
  }
</script>

<TauriEventListener eventName={TAURI_EVENTS.SHOW_SETTINGS} callback={() => showSettings = true} />
<WindowEvent callback={closeMenu} event="click" />
{#if showSettings}
  <Settings on:close={closeSettings} />
{/if}
<div class="flex items-center gap-1 mx-2">
  <Icon alt="scribe" src="/assets/logo.png" width="24" />
  {#each menubar as menu}
    <div class="relative">
      <button
        class="hover:bg-gray-300 px-2 rounded"
        on:click|stopPropagation={() => {
          activeMenu = menu.name;
        }}
      >
        <Typography variant="div" weight="normal" fontSize="base">
          {menu.name}
        </Typography>
      </button>
      <MenuItems
        on:close={closeMenu}
        items={menu.items}
        show={activeMenu === menu.name}
      />
    </div>
  {/each}
</div>
