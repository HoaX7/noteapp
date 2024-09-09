<script lang="ts">
	import StarterKit from '@tiptap/starter-kit';
	import { Editor } from '@tiptap/core';
	import { onDestroy, onMount } from 'svelte';
	import BubbleMenuComponent from './BubbleMenu.svelte';
	import extensions, { StarterKitOptions } from './extensions/index';
	import { BubbleMenu } from '@tiptap/extension-bubble-menu';
	import CustomMenu from './SlashCommandMenu.svelte';
	import { SlashCommands } from './extensions/slashCommands';

	let editor: Editor;
	let editorContainer: HTMLDivElement;
	let bubbleMenuEl: HTMLDivElement;
	let slashCommandEl: HTMLDivElement;

	onMount(() => {
		editor = new Editor({
			element: editorContainer,
			extensions: [
				...extensions,
				/**
				 * For further enhancements - 'history' ext can be used as a premium feature to
				 * track different versions.
				 */
				StarterKit.configure(StarterKitOptions),
				BubbleMenu.configure({
					element: bubbleMenuEl
				}),
				SlashCommands.configure({ element: slashCommandEl })
			],
			onTransaction() {
				editor = editor;
			},
			autofocus: true,
			injectCSS: false,
			editorProps: {
				attributes: {
					class: 'focus:outline-none',
					spellcheck: 'true'
				}
			}
		});
	});

	onDestroy(() => {
		editor?.destroy();
	});
</script>

<div id="editor" bind:this={editorContainer}></div>
<div bind:this={bubbleMenuEl}>
{#if editor}
	<BubbleMenuComponent {editor} />
{/if}
</div>
<div bind:this={slashCommandEl}>
{#if editor}
	<CustomMenu {editor} on:click={() => {
		slashCommandEl?.remove();
	}} />
{/if}
</div>