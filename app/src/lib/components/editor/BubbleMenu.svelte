<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import Typography from '../common/Typography.svelte';
	import MenuButton from './MenuButton.svelte';
	import Image from '../common/Icon.svelte';
	import clsx from 'clsx';

	export let editor: Editor;

	const menuItems = [
		{
			name: 'highlight',
			key: 'highlight',
			icon: 'assets/images/marker.svg',
			click: () => editor.chain().focus().toggleHighlight().run()
		},
		{
			name: '</>',
			key: 'code',
            icon: "assets/images/code.svg",
			click: () => editor.chain().focus().toggleCode().run()
		},
		{
			name: '>_',
			key: 'codeBlock',
            icon: "assets/images/codeblock.svg",
			click: () => editor.chain().focus().toggleCodeBlock().run()
		},
        {
            name: "Link",
            key: "link",
            icon: "assets/images/link.svg",
            click: () => showLinkInput = true
        },
        {
			name: 'B',
			key: 'bold',
			click: () => editor.chain().focus().toggleBold().run()
		},
		{
			name: 'I',
			key: 'italic',
			click: () => editor.chain().focus().toggleItalic().run()
		},
		{
			name: 'U̲',
			key: 'underline',
			click: () => editor.chain().focus().toggleUnderline().run()
		},
		{
			name: 'Strike',
			key: 'strike',
            icon: "assets/images/strikethrough.svg",
			click: () => editor.chain().focus().toggleStrike().run(),
            classname: "!p-1",
            w: 15
		},
		{
			name: 'Sup',
			key: 'superscript',
            icon: "assets/images/superscript.svg",
			click: () => editor.chain().focus().toggleSuperscript().run()
		},
		{
			name: 'Sub',
			key: 'subscript',
            icon: "assets/images/subscript.svg",
			click: () => editor.chain().focus().toggleSubscript().run(),
            classname: "!mr-[2px]"
		},
	];

	let showLinkInput = false;
	let link = editor?.getAttributes('link')?.href || '';

	const unlink = () => editor.chain().focus().extendMarkRange('link').unsetLink().run();
	const handleLinkInput = (e: any) => {
		link = e.target.value;
		if (e.code === 'Enter') {
			editor
				.chain()
				.focus()
				.extendMarkRange('link')
				.setLink({ href: link, target: '_blank', rel: 'noopener noreferrer nofollow' })
				.run();
			showLinkInput = false;
		} else if ((e.code === 'Backspace' || e.code === 'Enter') && link === '') {
			unlink();
			showLinkInput = false;
		}
	};
</script>

<div class={'bg-white rounded-md border shadow-lg flex items-center justify-center'}>
	{#if showLinkInput}
		<div class="rounded-md p-1 px-2">
			<input
				value={link}
				type="url"
				placeholder="link"
				class="focus:outline-none"
				on:keydown={handleLinkInput}
			/>
			<button
				class=""
				on:click={() => {
					showLinkInput = false;
				}}
			>
				&times;
			</button>
		</div>
	{:else}
		{#each menuItems as item}
			{#if item.icon}
				<MenuButton
					on:click={() => {
						item.click();
					}}
					classname={clsx('!p-0.5 block !ml-[2px] !m-0', item.classname)}
					isActive={editor.isActive(item.key || item.name.toLowerCase())}
				>
					<Image src={item.icon} alt={item.key} width={item.w || 18} />
				</MenuButton>
            {:else} 
            <MenuButton
				on:click={() => {
					item.click();
				}}
				isActive={editor.isActive(item.key || item.name.toLowerCase())}
			>
				<Typography fontSize="sm" weight="bold" variant="span">{item.name}</Typography>
			</MenuButton>
			{/if}
		{/each}
	{/if}
</div>
