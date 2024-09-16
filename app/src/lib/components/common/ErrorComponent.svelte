<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import BadgeAlert from "./Alert.svelte";
  import Typography from "./Typography.svelte";

  export let error = "";
  export let isModal = false;
  let timeout: NodeJS.Timeout;

  onMount(() => {
    if (isModal) {
      timeout = setTimeout(() => {
        error = "";
        isModal = false;
      }, 5000);
    }
  });
  onDestroy(() => clearTimeout(timeout));
</script>

{#if isModal}
  <BadgeAlert>
    <Typography
      variant="div"
      weight="normal"
      fontSize="sm"
      classname="text-red-600"
    >
      {error}
    </Typography>
  </BadgeAlert>
{:else}
  <Typography
    variant="div"
    weight="normal"
    fontSize="sm"
    classname="text-red-600"
  >
    {error}
  </Typography>
{/if}
