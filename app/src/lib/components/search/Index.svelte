<script lang="ts">
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";
  import { createEventDispatcher } from "svelte";
import Icon from "../common/Icon.svelte";
  import Modal from "../common/Modal.svelte";
  import Portal from "../common/Portal.svelte";
  import Typography from "../common/Typography.svelte";
  import Spinner from "../common/Spinner.svelte";

  export let showModal = false;
  export let loading = false;

  const dispatch = createEventDispatcher();
  const handleInput = (e: any) => {
    if (!loading) loading = true;
    dispatch("input", e.target.value);
  };

  const close = () => showModal = false;
</script>

<WindowEvent callback={close} event="click" />
<button class="px-3 my-1 w-full py-1" on:click|stopPropagation={() => showModal = true}>
    <Typography variant="div" weight="normal" fontSize="base" classname="flex gap-1">
        <Icon src="assets/images/search.svg" alt="search" width="16" /> Search
    </Typography>
</button>
{#if showModal}
    <Portal>
        <Modal on:close={close} empty containerClass="justify-items-center items-start pt-20">
            <div on:click|stopPropagation role="none">
                <diV class="p-3 border-b flex gap-1 rounded items-center">
                    <Icon src="assets/images/search.svg" width="24" alt="search" />
                    <input placeholder="Search" class="focus:outline-none w-full" on:input={handleInput} />
                    {#if loading}
                        <Spinner size="xxs" />
                    {/if}
                </diV>
                <div class="overflow-auto max-h-3/4 rounded">
                    <slot name="search-results" />
                </div>
            </div>
        </Modal>
    </Portal>
{/if}
