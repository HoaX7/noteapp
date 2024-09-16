<script lang="ts">
  import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();
</script>

<div class="absolute left-1/2 -translate-x-1/2 rounded-lg bg-gray-100 p-2">
    <slot />
</div>
