<script lang="ts">
  import Typography from "$lib/components/common/Typography.svelte";
  import Editor from "$lib/components/editor/Editor.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { saveContent } from "../../api";
  import clsx from "clsx";
  import ErrorComponent from "$lib/components/common/ErrorComponent.svelte";
  import { emit } from "@tauri-apps/api/event";
  import { Shortcuts, TAURI_EVENTS } from "../../utils/constants";
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";
  import SettingStore from "../../store/settings";

  const filename = "quicknotes";
  let saving = false;
  let previousContent = "";
  let error = ""
  const ctx = SettingStore.getContext();
  const TIME_GAP = 2; // time delay to group quicknotes under a timestamp.
  const appendDate = (text: string) => {
    const now = new Date().getTime();
    const appendDateAt = $ctx.lastQuickNoteAt
    let canAppendDate = !appendDateAt || now >= appendDateAt;
    if (canAppendDate) {
        const now = new Date().toLocaleString();
        const dt = new Date();
        ctx.update((state) => {
            state.lastQuickNoteAt = dt.setMinutes(dt.getMinutes() + TIME_GAP);
            return state;
        })
        return `${now}\n${text}`;
    }
    return text;
  }
  const handleSave = async (text: string) => {
    try {
        text = appendDate(text);
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

  const handleKeyPress = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "q") {
        appWindow.close();
    }
  }
</script>

<WindowEvent callback={handleKeyPress} event="keydown" />
<div class={clsx("border-b p-2 bg-white fixed h-8 user-select-none top-0 left-0 right-0",
    "flex items-center justify-between"
)} data-tauri-drag-region>
    {#if error}
        <ErrorComponent {error} isModal />
    {/if}
    <div class="flex gap-2">
        <Typography variant="div" weight="medium" fontSize="sm">
            Press <span class="rounded px-2 bg-gray-200">
                {Shortcuts($ctx.os).SAVE}
            </span> to save, <span class="rounded px-2 bg-gray-200">
                {Shortcuts($ctx.os).QUIT}
            </span> to quit 
        </Typography>
        {#if saving}
            <Typography variant="div" weight="normal" fontSize="sm" classname="bg-gray-200 px-2 rounded">
                saving...
            </Typography>
        {/if}
    </div>
    <button class="hover:bg-gray-200 text-black py-0 px-2 rounded-lg"
        on:click={() => {
            // save data to quicknotes file and close
            appWindow.close();
        }}
    >
        &times;
    </button>
</div>
<div class="mt-10 p-2"><Editor onData={handleSave} editorOptions={{}} /></div>