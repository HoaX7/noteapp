import { mergeAttributes, Node } from "@tiptap/core";
import { PluginKey } from "@tiptap/pm/state";
import Suggestion, { type SuggestionOptions } from "@tiptap/suggestion";
import type { Instance, Props } from "tippy.js";
import tippy from "tippy.js";

type SlashCommandOptions = {
    element: HTMLElement | null;
    tippy?: Instance<Props>;
    setAttr?: () => void;
}
const EXT_NAME = "SlashCommands";
const CHAR = "/";
export const SlashCommandPluginKey = new PluginKey(EXT_NAME);
export const SlashCommands = Node.create<SlashCommandOptions>({
        name: EXT_NAME,
        selectable: false,
        group: "inline",
        inline: true,
        atom: true,
        addOptions() {
            return {
                element: null
            }
        },
        addProseMirrorPlugins() {
            const popupElement = this.options.element;
            const { element: editorElement } = this.editor.options;
            if (!popupElement) return [];
            /**
             * detach the element from its current parent to
             * initially hide the component.
             */
            popupElement.remove();
            popupElement.style.visibility = 'visible';
            return [
                Suggestion({
                    editor: this.editor,
                    char: CHAR,
                    pluginKey: SlashCommandPluginKey,
                    allowSpaces: false,
                    /**
                     * We only show the suggestion if ('/') is typed.
                     */
                    allow: ({ state, range, editor }) => {
                        const node = state.selection.$from.node();
                        if (!node) return false;
                        return node.textBetween(0, 1) === CHAR;
                    },
                    render: () => {
                        return {
                            onStart: () => {
                                this.options.tippy = tippy(editorElement, {
                                    interactive: true,
                                    content: popupElement,
                                    trigger: "manual",
                                    placement: "bottom-start",
                                    getReferenceClientRect: null,
                                    showOnCreate: true,
                                })
                            },
                            onKeyDown: (props) => {
                                if (props.event.key === "Escape") {
                                    this.options.tippy?.hide();
                                    return true;
                                }
                                return false;
                            },
                            onExit: () => {
                                this.options.tippy?.destroy();
                                popupElement.remove();
                            },
                            onUpdate(props) {
                                console.log("Updated...")
                            },
                        }
                    },
                })
            ];
        },
    })
