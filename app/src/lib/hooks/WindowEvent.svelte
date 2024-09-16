<script lang="ts">
    import { onDestroy, onMount } from "svelte";
  
      export let callback: (ev: any) => void;
      export let event: keyof WindowEventMap;
  
      onMount(() => {
          window.addEventListener(event, callback);
      });
  
      onDestroy(() => {
          window.removeEventListener(event, callback);
      });
  
  </script>