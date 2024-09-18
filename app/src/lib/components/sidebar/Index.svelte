<script lang="ts">
  import Typography from "../common/Typography.svelte";
  import contextStore from "../../../store/context";
  import type { ContextProps } from "../../../types";
  import { onMount } from "svelte";
  import { getFileList } from "../../../api";
  import FileListItem from "./FileListItem.svelte";
  import { parseFilename } from "../../../utils";
  import Error from "../common/ErrorComponent.svelte";
  import TauriEventListener from "$lib/hooks/TauriEventListener.svelte";
  import { TAURI_EVENTS } from "../../../utils/constants";
  import SearchContent from "./SearchContent.svelte";

  let pages: ContextProps[] = [];
  let error = "";

  const ctx = contextStore.getContext();
  const getPageObject = (fn: string, ext = "md", isShortNote = false) => ({
    page: fn,
    ext,
    isShortNote
  })

  onMount(() => {
    loadData();
  });

  const updateStore = (selected: ContextProps) => {
    contextStore.update(selected);
  }

  const addNewPage = () => {
    let newPage = getPageObject("untitled");
    const found = pages.filter(({ page, ext }) => page.toLowerCase().startsWith(newPage.page) && ext === newPage.ext);
    if (found) {
      newPage.page = `${newPage.page}-${found.length}`;
    }
    updateStore(newPage);
    pages.splice(0, 0, newPage);
    pages = pages;
  }

  const loadData = async (canUpdate = true, forceUpdate = false) => {
    try {
    const result = await getFileList();
      pages = await Promise.all(result.map(async (page) => {
        const { filename, ext } = await parseFilename(page);
        const isShortNote = filename.toLowerCase() === "shortnotes";
        return getPageObject(filename, ext, isShortNote);
      }));
      if (canUpdate) {
        const selected = pages[0];
        if (selected?.page !== $ctx.page || forceUpdate) updateStore(selected);
      }
    } catch (err: any) {
      console.error("error", err);
      error = err?.message || "unable to load files";
    }
  }

  const handleRefresh = ({ payload }: { payload: { page: string; }; }) => {
    const found = pages.find((p) => p.page.toLowerCase() === payload.page.toLowerCase());
    if (found) return;
    loadData(pages.length <= 0);
  }

  const handlePageSelect = ({ detail }: { detail: string }) => {
    const found = pages.find((p) => `${p.page}.${p.ext}` === detail);
    if (found && ($ctx.page !== found.page || found.ext !== $ctx.ext)) {
      updateStore(found);
    }
  }
</script>

<TauriEventListener eventName={TAURI_EVENTS.RELOAD_FILES} callback={() => loadData(true, true)} />
<TauriEventListener eventName={TAURI_EVENTS.REFRESH_NOTES} callback={handleRefresh} />
{#if error}
  <Error {error} isModal />
{/if}
<div class="bg-gray-50 flex flex-col sidebar h-full border-r border-gray-200">
  <SearchContent on:pageSelect={handlePageSelect} />
  <div class="h-full border-b border-gray-200 overflow-auto">
    <ul>
      {#each pages as item, idx}
        <FileListItem {item} {updateStore} ctx={$ctx} {pages}
        setPages={(newPages) => {
          pages = newPages;
        }} />
      {/each}
    </ul>
  </div>
  <button
    class="text-start"
    on:click={addNewPage}
  >
    <Typography
      variant="div"
      weight="normal"
      fontSize="base"
      classname="px-3 py-1"
    >
      <Typography variant="span" weight="medium" fontSize="2xl">+</Typography> New
      note
    </Typography>
  </button>
</div>
