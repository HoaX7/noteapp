<script>
	import Sidebar from "$lib/components/Sidebar.svelte";
    import "../app.css";
</script>
  
<div class="flex h-full">
    <div class="w-80"><Sidebar /></div>
    <div class="p-3 overflow auto w-full"><slot /></div>
</div>
