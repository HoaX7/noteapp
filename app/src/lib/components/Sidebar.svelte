<script lang="ts">
	import Typography from "./common/Typography.svelte";

    let pages: string[] = ["untitled"];
</script>

<div class=" bg-gray-50 flex flex-col sidebar h-full border-r border-gray-200">
    <div class="border-b border-gray-200 px-3 py-2">
        <Typography
        variant="div"
        weight="normal"
        fontSize="base"
    >
        Starred
    </Typography>
    </div>
    <div class="h-full px-3 py-2 border-b border-gray-200 overflow-auto">
        {#each pages as page}
            <Typography
                variant="div"
                weight="normal"
                fontSize="base"
            >
                {page}
            </Typography>
        {/each}
    </div>
    <Typography
        variant="div"
        weight="normal"
        fontSize="base"
        classname="px-3 py-2"
    >
        New note
    </Typography>
</div>
