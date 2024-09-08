<script lang="ts">
	import StarterKit from '@tiptap/starter-kit';
	import { Editor } from '@tiptap/core';
	import { onDestroy, onMount } from 'svelte';
	import Placeholder from '@tiptap/extension-placeholder';
	import Document from '@tiptap/extension-document'
	import { StarterKitOptions } from './extensions';
	import CodeBlockLowLight from "@tiptap/extension-code-block-lowlight";
	import lowlight from "./registerCodeExt";
	import TaskList from "@tiptap/extension-task-list"
	import TaskItem from '@tiptap/extension-task-item';
	import codeblockNodeView from './codeblockNodeView';

	let editor: Editor;
	let editorContainer: HTMLDivElement;

	onMount(() => {
		editor = new Editor({
			element: editorContainer,
			extensions: [
				/**
				 * For further enhancements - 'history' ext can be used as a premium feature to
				 * track different versions.
				*/
				StarterKit.configure(StarterKitOptions),
				Placeholder.configure({
					placeholder: 'Write something ..'
				}),
				CodeBlockLowLight.extend({
					addNodeView() {
						return codeblockNodeView;
					},
				}).configure({ lowlight }),
				TaskList,
				TaskItem.configure({
					nested: true,
				}),
			],
			onTransaction() {
				editor = editor;
			},
			autofocus: true,
			injectCSS: false,
			editorProps: {
				attributes: {
					class: "focus:outline-none",
					spellcheck: "false",
				}
			},
		});
	});

	onDestroy(() => {
		editor?.destroy();
	});
</script>

<div id="editor" bind:this={editorContainer}></div>
