<script lang="ts">
  import StarterKit from "@tiptap/starter-kit";
  import { Editor, type EditorEvents, type EditorOptions } from "@tiptap/core";
  import { onDestroy, onMount } from "svelte";
  import BubbleMenuComponent from "./BubbleMenu.svelte";
  import extensions, { StarterKitOptions } from "./extensions/index";
  import { BubbleMenu } from "@tiptap/extension-bubble-menu";
  import { SlashCommands } from "./extensions/slashCommands";
  import { debounce } from "../../../utils";
  import WindowEvent from "$lib/hooks/WindowEvent.svelte";

  let editor: Editor;
  let editorContainer: HTMLDivElement;
  let bubbleMenuEl: HTMLDivElement;

  export let content = "";
  let previousContent = "";
  export let onData: (data: string) => Promise<boolean> | boolean;
  export let editorOptions: Partial<EditorOptions> = {
      onUpdate: debounce((props: EditorEvents["update"]) => saveContent(props.editor.getHTML()), 1000),
    };

  const saveContent = async (text: string) => {
    if (previousContent === text) return;
    let res = await onData?.(text);
    previousContent = text;
    return res;
  }

  const handleManualSave = async (ev: KeyboardEvent) => {
    if ((ev.metaKey || ev.ctrlKey) && ev.key === "s") {
      ev.preventDefault();
      const clearContent = await saveContent(editor.getHTML());
      if (clearContent) editor.commands.clearContent(false);
    }
  }
  onMount(() => {
    editor = new Editor({
      element: editorContainer,
      content,
      extensions: [
        ...extensions,
        /**
         * For further enhancements - 'history' ext can be used as a premium feature to
         * track different versions.
         */
        StarterKit.configure(StarterKitOptions),
        BubbleMenu.configure({ element: bubbleMenuEl }),
        SlashCommands,
      ],
      onTransaction() {
        editor = editor;
      },
      autofocus: true,
      injectCSS: false,
      editorProps: {
        attributes: {
          class: "focus:outline-none",
          spellcheck: "true",
        },
      },
      ...editorOptions,
    });
  });

  onDestroy(() => {
    editor?.destroy();
  });

</script>

<WindowEvent event="keydown" callback={handleManualSave} />
<div id="editor" bind:this={editorContainer}></div>
<BubbleMenuComponent bind:ref={bubbleMenuEl} {editor} />
