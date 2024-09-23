<script lang="ts">
  import Editor from "$lib/components/editor/Editor.svelte";
  import { marked } from "marked";
  import { getContent, saveContent } from "../api";
  import ContextStore from "../store/context";
  import Typography from "$lib/components/common/Typography.svelte";
  import Icon from "$lib/components/common/Icon.svelte";
  import Spinner from "$lib/components/common/Spinner.svelte";
  import Alert from "$lib/components/common/Alert.svelte";
  import type { ContextProps } from "../types";
  import ErrorComponent from "$lib/components/common/ErrorComponent.svelte";
  import TauriEventListener from "$lib/hooks/TauriEventListener.svelte";
  import { TAURI_EVENTS } from "../utils/constants";

  const ctx = ContextStore.getContext()
  let content = "";
  let loading = false;
  let saving = false;
  let hasPage = false;
  let error = "";
  let editorComponentRef: any;
  
  const getPath = (ctxObj: ContextProps) => `${ctxObj.page}.${ctxObj.ext}`;

  const handleOnData = async (text: string) => {
    try {
      saving = true;
      const fp = getPath($ctx);
      await saveContent({ path: fp, text, append: false });
    } catch (err: any) {
      console.error("unable to save", err)
      error = err?.message || "unable to save";
    }
    saving = false;
    return false;
  }
  const loadData = async (path: string) => {
    try {
      loading = true;
      const result = await getContent(path);
      content = await marked(result || "");
    } catch (err: any) {
      console.error("unable to load data", err);
      error = err?.message || "unable to load"
    }
    loading = false;
  }

  $: {
    hasPage = $ctx?.page?.length > 0;
    hasPage && loadData(getPath($ctx));
  }

  const handleContentUpdate = (data: { payload: { text: string; page: string; }; }) => {
    if ($ctx?.page?.toLowerCase() !== data.payload.page.toLowerCase()) return;
    editorComponentRef?.appendContent?.(data.payload.text);
  }
</script>

<TauriEventListener eventName={TAURI_EVENTS.REFRESH_NOTES} callback={handleContentUpdate} />
{#if saving}
<Alert>
  <Typography variant="div" weight="normal" fontSize="sm" classname="flex gap-1">
    <Spinner size="xxs" /> Saving..
  </Typography>
</Alert>
{/if}
{#if error}
  <ErrorComponent {error} isModal />
{/if}
<div class="lg:max-w-[80%] lg:mx-auto">
  <Typography
  variant="div"
  weight="medium"
  fontSize="sm"
  classname="text-gray-300 gap-2 flex items-center h-8"
>
  <Icon src="assets/images/notepad.svg" alt="notepad" width="18" /> 
  {#if $ctx.isShortNote}
    <Typography variant="div" weight="medium" fontSize="xl">
      Take quicknotes from anywhere by pressing 'Ctrl+Space'
    </Typography>
    {:else}
    <Typography variant="div" weight="medium" fontSize="xl">
      {$ctx.page}
    </Typography>
  {/if}
</Typography>
<div class="my-3 mb-10">
  {#if !loading && hasPage}
    <Editor onData={handleOnData} content={content} bind:this={editorComponentRef} />
  {/if}
</div>
</div>