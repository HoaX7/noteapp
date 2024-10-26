<script lang="ts">
  import Typography from "$lib/components/common/Typography.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import type { SettingProps } from "../../../../types";
  import { saveSettings } from "../../../../api";
  import Spinner from "$lib/components/common/Spinner.svelte";
  import Error from "$lib/components/common/ErrorComponent.svelte";
  import { emit } from "@tauri-apps/api/event";
  import { TAURI_EVENTS } from "../../../../utils/constants";
  import SettingStore from "../../../../store/settings";
  import DropDown from "$lib/components/common/DropDown.svelte";

  export let settings: SettingProps;
  let saving = false;
  let error = "";
  const ctx = SettingStore.getContext();

  const showDialog = async () => {
    try {
      let dir = await open({
        directory: true,
        multiple: false,
      });
      saving = true;
      if (typeof dir === "string" && dir !== settings.save_files_to_dir) {
        await saveSettings({ save_files_to_dir: dir });
        settings.save_files_to_dir = dir;
        emit(TAURI_EVENTS.RELOAD_FILES);
      }
    } catch (err: any) {
      console.error("Unable to save settings", err);
      error = err?.message || "unable to save settings";
    }
    saving = false;
  };

  const fonts = [{
    name: "Default",
    key: "default"
  }, {
    name: "Inter",
    key: "inter"
  }];

  let selected = $ctx.defaultFont ? fonts[0] : fonts[1];

  const handleSelect = (e: { detail: any; }) => {
    const data = e.detail;
    let isDefault = false;
    if (data.key === fonts[0].key) {
      isDefault = true;
    }
    document.body.style.fontFamily = isDefault ? "unset" : "Inter";
    ctx.update((store) => {
      store.defaultFont = isDefault;
      return store;
    });
  }
</script>

<div class="relative">
  {#if error}
    <Error {error} isModal />
  {/if}
  <Typography variant="div" weight="normal" fontSize="base">
    Version: {$ctx.version || "N/A"}
  </Typography>
  <div class="mt-3">
    <Typography
    variant="div"
    weight="normal"
    fontSize="sm"
    classname="text-slate-400 flex gap-1"
  >
    Save files to
    {#if saving}
      <Spinner size="xxs" /> saving...
    {/if}
  </Typography>
  <Typography
    variant="div"
    weight="normal"
    fontSize="base"
    classname="px-2 py-1 mt-1 rounded-md border flex justify-between"
  >
    <div class="truncate">
      {settings.save_files_to_dir}
    </div>
    <button class="bg-black text-white rounded px-1" on:click={showDialog}>
      Change
    </button>
  </Typography>
  <Typography
    variant="div"
    weight="normal"
    fontSize="sm"
    classname="text-slate-400"
  >
    Note: You will need to manually move the files from your current dir to the
    new one.
  </Typography>
  </div>
  <div class="mt-3">
    <Typography
    variant="div"
    weight="normal"
    fontSize="sm"
    classname="text-slate-400 flex gap-1"
  >
    Font
  </Typography>
  <div class="mt-1">
    <DropDown items={fonts} selectedItem={selected} on:select={handleSelect} />
  </div>
  </div>
</div>
