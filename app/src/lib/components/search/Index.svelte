<script lang="ts">
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";
  import { createEventDispatcher } from "svelte";
import Icon from "../common/Icon.svelte";
  import Modal from "../common/Modal.svelte";
  import Portal from "../common/Portal.svelte";
  import Typography from "../common/Typography.svelte";

  export let showModal = false;

  const dispatch = createEventDispatcher();
  const handleInput = (e: any) => dispatch("input", e.target.value);

  const close = () => showModal = false;
</script>

<WindowEvent callback={close} event="click" />
<button class="px-3 my-1 hover:bg-gray-200 w-full" on:click|stopPropagation={() => showModal = true}>
    <Typography variant="div" weight="normal" fontSize="base" classname="flex gap-1">
        <Icon src="assets/images/search.svg" alt="search" width="16" /> Search
    </Typography>
</button>
{#if showModal}
    <Portal>
        <Modal on:close={close} empty containerClass="justify-items-center items-start pt-20">
            <div on:click|stopPropagation role="none">
                <diV class="p-3 border-b flex gap-1">
                    <Icon src="assets/images/search.svg" width="24" alt="search" />
                    <input placeholder="Search" class="focus:outline-none w-full" on:input={handleInput} />
                </diV>
                <div class="overflow-auto max-h-3/4">
                    <slot name="search-results" />
                </div>
            </div>
        </Modal>
    </Portal>
{/if}
