<script lang="ts">
  import clsx from "clsx";
  import type { ContextProps } from "../../../types";
  import Icon from "../common/Icon.svelte";
  import Typography from "../common/Typography.svelte";
  import { deleteFile, renameFile } from "../../../api";
  import Error from "../common/ErrorComponent.svelte";
  import { IO } from "../../../utils/errors";
  import { parseFileModifiedDate } from "../../../utils";
  import { showMenu } from "tauri-plugin-context-menu";

  export let item: ContextProps;
  export let ctx: ContextProps;
  export let updateStore: (item: ContextProps) => void;
  export let pages: ContextProps[] = [];
  export let setPages: (pages: ContextProps[]) => void;

  let allowEdit = false;
  let fileExists = false;
  let error = "";

  const updatePages = (obj: ContextProps) => {
    const idx = pages.findIndex((p) => p.page === obj.page && p.ext === obj.ext);
    if (idx >= 0) {
        pages.splice(idx, 1);
        setPages(pages);
    }
    if (ctx.page === obj.page && ctx.ext === obj.ext) {
        updateStore(pages[0] || {});
    }
  }
  const deletePage = async (obj: ContextProps) => {
    const filename = `${obj.page}.${obj.ext}`;
    try {
        await deleteFile(filename);
        updatePages(obj);
    } catch (err: any) {
        if (err.code === IO.IO_NF) {
            updatePages(obj);
            return;
        }
        console.error("unable to delete", err);
        error = err?.message || "unable to delete page";
    }
  };
  const setEditable = (bool: boolean) => {
    allowEdit = bool;
  }

  const handleRename = async (e: any) => {
    const value = e.target.value;
    const denyRename = fileExists || e.code !== "Enter" || value === "";
    if (denyRename || (item.page.toLowerCase() === value.toLowerCase())) return;
    try {
        await renameFile(item.page, value);
        const idx = pages.findIndex((p) => p.page === item.page && p.ext === item.ext);
        if (idx >= 0) {
            pages[idx].page = value;
            setPages(pages);
        }
        if (ctx.page === item.page && ctx.ext === item.ext) {
            updateStore({
                ...ctx,
                page: value
            });
        }
        setEditable(false);
    } catch (err: any) {
        console.error("unable to rename file", err);
        error = err?.message || "unable to rename";
    }
  }

  const handleInput = (e: any) => {
    const value = e.target.value;
    fileExists = pages.some(({ page }) => page.toLowerCase() === value.toLowerCase());
  }
  const handleDelPage = async () => {
    const ask = await window.confirm("Are you sure you want to delete this file? This action cannot be reverted.");
    if (!ask) return;
    deletePage(item);
  }

  const onContextMenu = (e: any) => {
    const browser = {
      w: window.innerWidth,
      h: window.innerHeight
    };

    showMenu({
      pos: {
        x: e.clientX,
        y: e.clientY
      },
      items: [{
        label: "Rename",
        disabled: item.isShortNote,
        payload: item.page,
        event: async () => {
          if (item.isShortNote) return;
          setEditable(true);
        }
      }, {
        label: "Delete",
        disabled: item.isShortNote,
        payload: item.page,
        event: () => {
          if (item.isShortNote) return;
          handleDelPage();
        }
      }]
    })
  }
</script>

<li
  class={clsx(
    "px-3 cursor-pointer justify-between items-center",
    ctx.page === item.page ? "bg-gray-200" : "",
    "relative border-b p-2 rounded-md"
  )}
  on:click={() => {
    updateStore?.(item);
  }}
  role="none"
  on:contextmenu|preventDefault={onContextMenu}
>
  <div class="flex gap-1 items-center truncate">
    <Icon src="assets/images/page.svg" alt="page" width="14" />
    <Typography variant="div" weight="normal" fontSize="base">
      {item.page}.{item.ext}
    </Typography>
  </div>
  <Typography variant="div" weight="medium" fontSize="sm" classname="text-gray-500">
    {parseFileModifiedDate(item.modified)}
  </Typography>
  {#if allowEdit}
    <div class={clsx("z-10 bg-white rounded-md shadow-lg top-5 m-2 absolute border p-1 px-2",
        "flex items-center gap-1"
    )} 
        on:click|stopPropagation={() => {}}
        role="none">
        <div>
            <input maxlength="100" value={item.page} class="w-full" placeholder="start typing.." 
                on:keypress={handleRename}
                on:input={handleInput}
            />
            <Error error={fileExists ? "File exists" : ""} />
        </div>
        <button on:click|stopPropagation={() => setEditable(false)}><Icon src="assets/images/close.svg" alt="close" /></button>
    </div>
  {/if}
</li>
