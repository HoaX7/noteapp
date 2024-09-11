<script lang="ts">
  import Typography from "./common/Typography.svelte";
  import Icon from "../components/common/Icon.svelte";
  import contextStore from "../../store/context";
  import clsx from "clsx";
  import shortid from "shortid";
  import type { ContextProps } from "../../types";

  let pages: ContextProps[] = [];

  const ctx = contextStore.getContext();

  const addNewPage = () => {
    const newPage = { id: shortid(), page: "untitled" };
    contextStore.update(newPage);
    pages.splice(0, 0, newPage);
    pages = pages;
  }
</script>

<div class="bg-gray-50 flex flex-col sidebar h-full border-r border-gray-200">
  <div class="border-b border-gray-200 px-3 py-2">
    <Typography variant="div" weight="normal" fontSize="base">Starred</Typography
    >
  </div>
  <div class="h-full border-b border-gray-200 overflow-auto">
    <ul>
      {#each pages as item}
        <li class={clsx("px-3 my-2 hover:bg-gray-200 cursor-pointer flex items-center gap-1",
          $ctx.id === item.id ? "bg-gray-200" : ""
        )}>
          <Icon 
            src="assets/images/page.svg"
            alt="page"
            width="14"
          />
          <Typography variant="span" weight="normal" fontSize="base">
            {item.page}
          </Typography>
        </li>
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
