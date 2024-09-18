<script lang="ts">
  import Typography from "$lib/components/common/Typography.svelte";
  import Editor from "$lib/components/editor/Editor.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { saveContent } from "../../api";
  import clsx from "clsx";
  import ErrorComponent from "$lib/components/common/ErrorComponent.svelte";
  import ContextStore from "../../store/context";
  import { emit } from "@tauri-apps/api/event";
  import { TAURI_EVENTS } from "../../utils/constants";

  const filename = "shortnotes";
  let saving = false;
  let previousContent = "";
  let error = ""
  const ctx = ContextStore.getContext();
  const handleSave = async (text: string) => {
    try {
        if (previousContent === text) return false;
        saving = true;
        await saveContent({
            path: filename,
            text,
            append: true
        });
        previousContent = text;
        await emit(TAURI_EVENTS.REFRESH_NOTES, { text, page: filename });
        saving = false;
        return true;
    } catch (err: any) {
        console.error("unable to save:", err)
        error = err?.message || "unable to save";
    }
    saving = false;
    return false;
  }
</script>

<div class={clsx("border-b p-2 bg-white fixed h-8 user-select-none top-0 left-0 right-0",
    "flex items-center justify-between"
)} data-tauri-drag-region>
    {#if error}
        <ErrorComponent {error} isModal />
    {/if}
    <div class="flex gap-2">
        <Typography variant="div" weight="medium" fontSize="sm">
            Press <span class="rounded px-2 bg-gray-200">ctrl+s</span> to save
        </Typography>
        {#if saving}
            <Typography variant="div" weight="normal" fontSize="sm" classname="bg-gray-200 px-2 rounded">
                saving...
            </Typography>
        {/if}
    </div>
    <button class="hover:bg-gray-200 text-black py-0 px-2 rounded-lg"
        on:click={() => {
            // save data to shortnotes file and close
            appWindow.close();
        }}
    >
        &times;
    </button>
</div>
<div class="mt-10 p-2"><Editor onData={handleSave} editorOptions={{}} /></div>