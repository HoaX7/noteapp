import Placeholder from '@tiptap/extension-placeholder';
import CodeBlockLowLight from '@tiptap/extension-code-block-lowlight';
import lowlight from './registerCodeExt';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';
import codeblockNodeView from '../codeblockNodeView';
import Link from '@tiptap/extension-link';
import Subscript from '@tiptap/extension-subscript';
import Superscript from '@tiptap/extension-superscript';
import Underline from '@tiptap/extension-underline';
import Highlight from '@tiptap/extension-highlight';

export const StarterKitOptions = {
    blockquote: {
        HTMLAttributes: {
            class: "pl-2 my-2 border-l-2 border-gray-300"
        }
    },
    listItem: {
        HTMLAttributes: {
            class: "my-1"
        }
    },
    orderedList: {
        HTMLAttributes: {
            class: "px-2 mx-2 my-2"
        }
    },
    bulletList: {
        HTMLAttributes: {
            class: "px-2 mx-2 my-2"
        }
    },
    code: {
        HTMLAttributes: {
            class: "px-1 bg-gray-200 rounded"
        }
    },
    horizontalRule: {
        HTMLAttributes: {
            class: "my-8"
        }
    },
    codeBlock: false as false,
}

export default [
    Placeholder.configure({
        placeholder: "Write something, or press '/' for commands..."
    }),
    CodeBlockLowLight.extend({
        addNodeView() {
            return codeblockNodeView;
        }
    }).configure({ lowlight }),
    TaskList,
    TaskItem.configure({
        nested: true
    }),
    Link.configure({
        openOnClick: false,
        autolink: true,
        defaultProtocol: 'https',
        HTMLAttributes: {
            class: 'underline text-blue-600'
        }
    }),
    Subscript,
    Superscript,
    Underline,
    Highlight,
]
