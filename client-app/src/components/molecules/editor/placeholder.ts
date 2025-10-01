import { Node, mergeAttributes } from "@tiptap/core";

export interface PlaceholderOptions {
  HTMLAttributes: Record<string, any>;
}

declare module "@tiptap/core" {
  interface Commands<ReturnType> {
    placeholder: {
      insertPlaceholder: (name: string) => ReturnType;
    };
  }
}

export const Placeholder = Node.create<PlaceholderOptions>({
  name: "placeholder",

  group: "inline",
  inline: true,
  atom: true,

  addOptions() {
    return {
      HTMLAttributes: {
        class: "placeholder",
      },
    };
  },

  addAttributes() {
    return {
      name: {
        default: null,
        parseHTML: (element) => element.getAttribute("data-name"),
        renderHTML: (attributes) => {
          return { "data-name": attributes.name };
        },
      },
    };
  },

  parseHTML() {
    return [{ tag: "span[data-placeholder]" }];
  },

  renderHTML({ node, HTMLAttributes }) {
    return [
      "span",
      mergeAttributes(this.options.HTMLAttributes, HTMLAttributes, {
        "data-placeholder": node.attrs.name,
      }),
      `{${node.attrs.name}}`,
    ];
  },

  addCommands() {
    return {
      insertPlaceholder:
        (name: string) =>
        ({ chain }) => {
          return chain().insertContent({ type: this.name, attrs: { name } }).run();
        },
    };
  },
});
