<script lang="ts">
  import Typography from "../common/Typography.svelte";
  import contextStore from "../../../store/context";
  import type { ContextProps } from "../../../types";
  import { onMount } from "svelte";
  import { getFileList } from "../../../api";
  import FileListItem from "./FileListItem.svelte";
  import { getFullMonth, parseFilename } from "../../../utils";
  import Error from "../common/ErrorComponent.svelte";
  import TauriEventListener from "$lib/hooks/TauriEventListener.svelte";
  import { TAURI_EVENTS } from "../../../utils/constants";
  import SearchContent from "./SearchContent.svelte";

  type GroupedPages = {
    [key: string]: ContextProps[];
  };
  let pages: ContextProps[] = [];
  let error = "";

  const ctx = contextStore.getContext();
  const getPageObject = (
    fn: string,
    ext = "md",
    isShortNote = false,
    modified = new Date()
  ) => ({
    page: fn,
    ext,
    isShortNote,
    modified,
  });

  onMount(() => {
    loadData();
  });

  const updateStore = (selected: ContextProps) => {
    contextStore.update(selected);
  };

  const addNewPage = () => {
    let newPage = getPageObject("untitled");
    const found = pages.filter(
      ({ page, ext }) =>
        page.toLowerCase().startsWith(newPage.page) && ext === newPage.ext
    );
    if (found) {
      newPage.page = `${newPage.page}-${found.length}`;
    }
    updateStore(newPage);
    pages.splice(0, 0, newPage);
    pages = pages;
  };

  const loadData = async (canUpdate = true, forceUpdate = false) => {
    try {
      const result = await getFileList();
      pages = await Promise.all(
        result
          .sort((a, b) => (a.modified < b.modified ? 1 : -1))
          .map(async (page) => {
            const { filename, ext } = await parseFilename(page.filename);
            const isQuickNote = filename.toLowerCase() === "quicknotes";
            return getPageObject(
              filename,
              ext,
              isQuickNote,
              new Date(page.modified)
            );
          })
      );
      if (canUpdate) {
        const selected = pages[0];
        if (selected?.page !== $ctx.page || forceUpdate) updateStore(selected);
      }
    } catch (err: any) {
      console.error("error", err);
      error = err?.message || "unable to load files";
    }
  };

  const handleRefresh = ({ payload }: { payload: { page: string } }) => {
    const found = pages.find(
      (p) => p.page.toLowerCase() === payload.page.toLowerCase()
    );
    if (found) return;
    loadData(pages.length <= 0);
  };

  const handlePageSelect = ({ detail }: { detail: string }) => {
    const found = pages.find((p) => `${p.page}.${p.ext}` === detail);
    if (found && ($ctx.page !== found.page || found.ext !== $ctx.ext)) {
      updateStore(found);
    }
  };

  let groupedPages: GroupedPages = {};
  let groupedPagesKeys: string[] = [];
  const dt = new Date();
  const curyear = dt.getFullYear();
  const curdate = dt.getDate();
  const curmonth = dt.getMonth();
  $: {
    groupedPages = pages.reduce((acc, r) => {
      const [yy, mm, dd] = [
        r.modified.getFullYear(),
        r.modified.getMonth(),
        r.modified.getDate(),
      ];
      let key: any = yy;
      if (curyear === yy) {
        if (dd === curdate && curmonth === mm) {
          key = "Today";
        } else {
          key = getFullMonth(mm);
        }
      }
      acc[key] = (acc[key] || []).concat(r);
      return acc;
    }, {} as GroupedPages);
    groupedPagesKeys = Object.keys(groupedPages).sort((a, b) =>
      a < b ? 1 : -1
    );
  }

  const getPages = (label: string) => {
    return groupedPages[label as keyof GroupedPages];
  };

  const handlePageModified = ({ payload }: { payload: { page: string }; }) => {
    const idx = pages.findIndex((p) => p.page === payload.page);
    if (idx >= 0) {
      pages[idx].modified = new Date();
      pages = pages;
    }
  }
</script>

<TauriEventListener
  eventName={TAURI_EVENTS.RELOAD_FILES}
  callback={() => loadData(true, true)}
/>
<TauriEventListener
  eventName={TAURI_EVENTS.REFRESH_NOTES}
  callback={handleRefresh}
/>
<TauriEventListener 
  eventName={TAURI_EVENTS.PAGE_MODIFIED}
  callback={handlePageModified}
/>
{#if error}
  <Error {error} isModal />
{/if}
<div class="bg-gray-50 flex flex-col h-full border-r border-gray-200">
  <div class="flex-shrink-0">
    <SearchContent on:pageSelect={handlePageSelect} />
  </div>
  <div class="flex-grow overflow-auto h-[200px]">
    {#each groupedPagesKeys as label}
      <div class="mb-2">
        <Typography
          variant="div"
          weight="medium"
          fontSize="sm"
          classname="text-gray-500 px-3 p-2"
        >
          {label}
        </Typography>
        <ul class="file-list-wrapper mx-2">
          {#each getPages(label) as item}
            <FileListItem
              {item}
              {updateStore}
              ctx={$ctx}
              {pages}
              setPages={(newPages) => {
                pages = newPages;
              }}
            />
          {/each}
        </ul>
      </div>
    {/each}
  </div>
  <button class="flex-shrink-0 text-start mt-auto border-t border-gray-200" on:click={addNewPage}>
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
