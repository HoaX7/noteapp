<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Typography from "./Typography.svelte";

    export let items: {
        key: string;
        name: string;
    }[] = [];
    export let showDD = false;
    export let selectedItem = "Options";
    const dispatch = createEventDispatcher();
</script>

<div class="rounded bg-gray-200">
    <Typography variant="div" weight="normal" fontSize="base">
        {selectedItem}
    </Typography>
</div>
