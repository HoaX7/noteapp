<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import clsx from "clsx";
  import Icon from "../common/Icon.svelte";
  import MainMenu from "../menu/Main.svelte";

  const titlebar = [
    {
      name: "minimize",
      action: appWindow.minimize,
      icon: "/assets/images/minimize.svg",
    },
    {
      name: "maximize",
      action: appWindow.toggleMaximize,
      icon: "/assets/images/maximize.svg",
    },
    {
      name: "close",
      action: appWindow.hide,
      icon: "/assets/images/close.svg",
    },
  ];
</script>

<div
  data-tauri-drag-region
  class={clsx(
    "h-8 flex bg-gray-200 fixed user-select-none top-0 left-0 right-0",
    "items-center justify-between z-10"
  )}
>
  <MainMenu />
  <div>
    {#each titlebar as item}
      <button on:click={item.action} class="hover:bg-gray-300 p-2">
        <Icon alt={item.name} src={item.icon} />
      </button>
    {/each}
  </div>
</div>
