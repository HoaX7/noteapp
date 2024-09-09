<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let src: string;
	export let alt: string;

	const dispatch = createEventDispatcher();
</script>

<img
	{src}
	{alt}
	{...$$restProps}
	on:error={(e) => {
		dispatch('error', e);
	}}
/>
