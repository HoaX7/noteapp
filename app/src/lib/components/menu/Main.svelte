<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import Icon from "../common/Icon.svelte";
  import Typography from "../common/Typography.svelte";

    const menubar = [{
        name: "File",
        items: [{
            name: ""
        }]
    }]
</script>

<div class="flex items-center gap-1 mx-2">
    <Icon 
        alt="scribe"
        src="/assets/logo.svg"
        width="24"
    />
    <button class="hover:bg-gray-300 px-2 rounded">
        <Typography
        variant="div"
        weight="normal"
        fontSize="base"
    >
        File
    </Typography>
    </button>
</div>
