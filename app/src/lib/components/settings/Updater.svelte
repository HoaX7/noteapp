<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { installUpdates } from "../../../api/update";
import Spinner from "../common/Spinner.svelte";
import Typography from "../common/Typography.svelte";

    export let updateManifest;
    let saving = false;
    const dispatch = createEventDispatcher();

    const handleInstallUpdate = async () => {
        saving = true;
        try {
            // await installUpdates(updateManifest.manifest);
            dispatch("updatecomplete");
        } catch (err) {
            console.error("Unable to update", err)
        }
        saving = false;
    }
</script>

{#if updateManifest.updateAvailable}
<Typography classname="mx-3 text-white flex gap-2 items-center" variant="div" weight="normal" fontSize="base">
  v{updateManifest.manifest.version} available 
  {#if saving}
    <Spinner size="xxs" />
  {:else}
    <button class="bg-gray-200 px-2 text-xs rounded text-black"
        on:click={handleInstallUpdate}
    >install now</button>
  {/if}
</Typography>
{/if}