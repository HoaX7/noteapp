<script lang="ts">
	import { type Editor } from '@tiptap/core';
	import type { Level } from '@tiptap/extension-heading';
	import MenuButton from './MenuButton.svelte';
	import Typography from '../common/Typography.svelte';
	import { createEventDispatcher } from 'svelte';

	export let editor: Editor;
	const headerLevels: Level[] = [1, 2, 3, 4, 5, 6];
	const menuItems = [
		...headerLevels.map((level) => ({
			name: `Heading ${level}`,
			click: () => editor.chain().focus().toggleHeading({ level }).run()
		})),
		{
			name: 'Paragraph',
			click: () => editor.chain().focus().toggleNode('paragraph', 'paragraph').run()
		},
		{
			name: 'Bullet List',
			click: () => editor.chain().focus().toggleBulletList().run()
		},
		{
			name: 'Check List',
			click: () => editor.chain().focus().toggleTaskList().run()
		},
		{
			name: 'Ordered List',
			click: () => editor.chain().focus().toggleOrderedList().run()
		},
		{
			name: 'Code Block',
			click: () => editor.chain().focus().toggleCodeBlock().run()
		},
		{
			name: 'Block Quotes',
			click: () => editor.chain().focus().toggleBlockquote().run()
		}
	];

    const dispatch = createEventDispatcher();

    const clearBlock = () => {
        const selection = editor.state.selection;
        editor.chain().focus().insertContentAt({
            from: selection.from - 1,
            to: selection.to
        }, "").run();
    }
</script>

<div class="flex flex-col border shadow bg-white rounded-md">
    {#each menuItems as item}
        <MenuButton on:click={() => {
            item.click();
            /**
             * When an item is selected, hide the element.
            */
            dispatch("click");
            clearBlock();
        }}>
            <Typography
                variant="span"
                weight="normal"
                fontSize="sm"
            >
                {item.name}
            </Typography>
        </MenuButton>
    {/each}
</div>