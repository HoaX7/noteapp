<script lang="ts">
  import { onMount } from "svelte";
  import { settingsMenu } from ".";
  import Modal from "../common/Modal.svelte";
  import Portal from "../common/Portal.svelte";
  import MenuItems from "../menu/MenuItems.svelte";
  import Home from "./tabs/Home.svelte";
  import { getSettings } from "../../../api";
  import type { SettingProps } from "../../../types";
  import Error from "../common/ErrorComponent.svelte";

  const tabs = [
    {
      name: "Home",
      component: Home,
    },
  ];
  let selectedTab = tabs[0];
  let settings = {} as SettingProps;
  let error = "";

  onMount(async () => {
    try {
      settings = await getSettings();
    } catch (err: any) {
      console.error("unable to load settings", err);
      error = err.message;
    }
  });
</script>

<Portal>
  <Modal on:close>
    {#if error}
      <Error {error} isModal />
    {/if}
    <div slot="title">Preferences</div>
    <div class="grid grid-cols-6 gap-4">
      <MenuItems
        containerClass="col-span-2"
        show
        selected={selectedTab.name}
        items={settingsMenu}
      />
      <div class="col-span-4">
        <svelte:component this={selectedTab.component} {settings} />
      </div>
    </div>
  </Modal>
</Portal>
