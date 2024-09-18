<script lang="ts">
  import Typography from "$lib/components/common/Typography.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import type { SettingProps } from "../../../../types";
  import { saveSettings } from "../../../../api";
  import Spinner from "$lib/components/common/Spinner.svelte";
  import Error from "$lib/components/common/ErrorComponent.svelte";
  import { emit } from "@tauri-apps/api/event";
  import { TAURI_EVENTS } from "../../../../utils/constants";

  export let settings: SettingProps;
  let saving = false;
  let error = "";

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
</script>

<div class="relative">
  {#if error}
    <Error {error} isModal />
  {/if}
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
