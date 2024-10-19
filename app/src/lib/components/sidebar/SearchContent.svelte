<script lang="ts">
  import { path } from "@tauri-apps/api";
  import { searchContent } from "../../../api";
  import { debounce } from "../../../utils";
  import ErrorComponent from "../common/ErrorComponent.svelte";
  import Search from "../search/Index.svelte";
  import Icon from "../common/Icon.svelte";
  import Typography from "../common/Typography.svelte";
  import { createEventDispatcher } from "svelte";
  import Spinner from "../common/Spinner.svelte";
  import Alert from "../common/Alert.svelte";
  import BubbleMenu from "../editor/BubbleMenu.svelte";

  let data: string[] = [];
  let error = "";
  let loading = false;
  let query = "";
  let showModal = false;

  const handleSearch = async (e: { detail: string }) => {
    query = e.detail;
    try {
      loading = true;
      error = "";
      const result = await searchContent(query);
      data = await Promise.all(result.map((fp) => path.basename(fp)));
    } catch (err: any) {
      error = err?.message || "unable to find content";
    }
    loading = false;
  };

  const dispatch = createEventDispatcher();
</script>

<div class="border-b">
  <Search bind:loading={loading} on:input={debounce(handleSearch, 1000)} bind:showModal={showModal}>
    <div slot="search-results">
      {#each data as item}
        <div
          class="p-3 hover:bg-gray- border-b cursor-pointer"
          role="none"
          on:click|stopPropagation={() => {
            dispatch("pageSelect", item);
            showModal = false;
            data = [];
            query = "";
          }}
        >
          <Typography
            variant="div"
            weight="semi-bold"
            fontSize="xl"
            classname="flex gap-1"
          >
            <Icon src="assets/images/page.svg" alt="file" width="18" />
            {item}
          </Typography>
          <Typography variant="div" weight="normal" fontSize="base">
            Found in file: <strong>{query}</strong>
          </Typography>
        </div>
      {/each}
      {#if error}
        <ErrorComponent {error} />
      {/if}
    </div>
  </Search>
</div>
