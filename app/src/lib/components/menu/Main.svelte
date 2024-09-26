<script lang="ts">
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";
  import ContextStore from "../../../store/context";
  import Icon from "../common/Icon.svelte";
  import Typography from "../common/Typography.svelte";
  import MenuItems from "./MenuItems.svelte";
  import { getMenuBar } from "./items";
  import { TAURI_EVENTS } from "../../../utils/constants";
  import SettingStore from "../../../store/settings";
  import { emit } from "@tauri-apps/api/event";

  const ctx = ContextStore.getContext();
  const settingCtx = SettingStore.getContext();
  let menubar = [];
  let activeMenu = "";

  const closeMenu = () => {
    activeMenu = "";
  };
  $: menubar = getMenuBar($ctx.page, $settingCtx.os, () => {
    emit(TAURI_EVENTS.SHOW_SETTINGS);
  });
</script>

<WindowEvent callback={closeMenu} event="click" />
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
