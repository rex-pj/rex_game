<script lang="ts">
  import StarterKit from "@tiptap/starter-kit";
  import { Editor } from "@tiptap/core";
  import { onMount } from "svelte";
  import Link from "@tiptap/extension-link";
  import { normalizeUrl } from "@sveltejs/kit";
  import { isLinkValid } from "$lib/helpers/linkHelper";

  let value: string = "";
  export { value as value };
  let element: Element;
  let editor: Editor;

  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [
        StarterKit,
        Link.extend({
          inclusive: false,
        }).configure({
          autolink: true,
          openOnClick: false,
          protocols: ["http", "https", "mailto"],
          HTMLAttributes: {
            rel: "noopener noreferrer",
            target: "_blank",
          },
        }),
      ],
      content: value,
      onUpdate: ({ editor }) => {
        const html = editor.getHTML();
        value = html;
      },
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor;
      },
    });
  });

  let headingLevel = 0;
  function setHeading(level: number) {
    headingLevel = level;
    editor
      .chain()
      .focus()
      .toggleHeading({ level: level as any })
      .run();
  }

  function isHeadingActive() {
    return [1, 2, 3, 4, 5, 6].some((level) => {
      return editor.isActive("heading", { level });
    });
  }

  function toggleChain(command: string) {
    switch (command) {
      case "bold":
        editor.chain().focus().toggleBold().run();
        break;
      case "italic":
        editor.chain().focus().toggleItalic().run();
        break;
      case "strike":
        editor.chain().focus().toggleStrike().run();
        break;
      case "code":
        editor.chain().focus().toggleCode().run();
        break;
      case "paragraph":
        editor.chain().focus().setParagraph().run();
        break;
      case "bulletList":
        editor.chain().focus().toggleBulletList().run();
        break;
      case "orderedList":
        editor.chain().focus().toggleOrderedList().run();
        break;
      case "codeBlock":
        editor.chain().focus().toggleCodeBlock().run();
        break;
      case "blockquote":
        editor.chain().focus().toggleBlockquote().run();
        break;
      case "horizontalRule":
        editor.chain().focus().setHorizontalRule().run();
        break;
      case "undo":
        editor.chain().focus().undo().run();
        break;
      case "redo":
        editor.chain().focus().redo().run();
        break;
    }
  }

  function canToggleChain(command: string) {
    switch (command) {
      case "bold":
        return editor.can().chain().focus().toggleBold().run();
      case "italic":
        return editor.can().chain().focus().toggleItalic().run();
      case "strike":
        return editor.can().chain().focus().toggleStrike().run();
      case "code":
        return editor.can().chain().focus().toggleCode().run();
      case "paragraph":
        return true;
      case "bulletList":
        return editor.can().chain().focus().toggleBulletList().run();
      case "orderedList":
        return editor.can().chain().focus().toggleOrderedList().run();
      case "codeBlock":
        return editor.can().chain().focus().toggleCodeBlock().run();
      case "blockquote":
        return editor.can().chain().focus().toggleBlockquote().run();
      case "horizontalRule":
        return editor.can().chain().focus().setHorizontalRule().run();
      case "undo":
        return editor.can().chain().focus().undo().run();
      case "redo":
        return editor.can().chain().focus().redo().run();
    }
  }

  function insertPlaceholder(placeholder: string) {
    editor.chain().focus().insertContent(`[${placeholder}]`).run();
  }

  function setLink(href: string, extra: Record<string, any> = {}) {
    if (!editor) {
      return;
    }

    const result = isLinkValid(href);
    if (!result.valid) {
      editor.chain().focus().extendMarkRange("link").unsetLink().run();
      alert(result.error);
      return;
    }

    const { url } = normalizeUrl(href);
    editor
      .chain()
      .focus()
      .extendMarkRange("link")
      .setLink({
        href: url.href,
        ...extra,
      })
      .run();
  }

  function unsetLink() {
    editor?.chain().focus().extendMarkRange("link").unsetLink().run();
  }

  function promptLink() {
    if (!editor) {
      return;
    }
    const current = editor.getAttributes("link")?.href ?? "";
    const input = window.prompt(current);
    if (!input) {
      return;
    }
    setLink(input);
  }
</script>

{#if editor}
  <div class="control-group">
    <div class="button-group">
      <button
        type="button"
        aria-label="Bold"
        title="Bold"
        onclick={() => toggleChain("bold")}
        disabled={!canToggleChain("bold")}
        class={editor.isActive("bold") ? "is-active" : ""}
      >
        <i class="fas fa-bold"></i>
      </button>
      <button
        type="button"
        aria-label="Italic"
        title="Italic"
        onclick={() => toggleChain("italic")}
        disabled={!canToggleChain("italic")}
        class={editor.isActive("italic") ? "is-active" : ""}
      >
        <i class="fas fa-italic"></i>
      </button>
      <button
        type="button"
        aria-label="Strike"
        title="Strike"
        onclick={() => toggleChain("strike")}
        disabled={!canToggleChain("strike")}
        class={editor.isActive("strike") ? "is-active" : ""}
      >
        <i class="fas fa-strikethrough"></i>
      </button>
      <button
        type="button"
        aria-label="Code"
        title="Code"
        onclick={() => toggleChain("code")}
        disabled={!canToggleChain("code")}
        class={editor.isActive("code") ? "is-active" : ""}
      >
        <i class="fas fa-code"></i>
      </button>
      <button
        type="button"
        aria-label="Paragraph"
        title="Paragraph"
        onclick={() => toggleChain("paragraph")}
        class={editor.isActive("paragraph") ? "is-active" : ""}
      >
        <i class="fas fa-paragraph"></i>
      </button>
      <!-- Dropdown cho Heading -->
      <div class="dropdown">
        <button class="dropdown-toggle {isHeadingActive() ? 'is-active' : ''}" type="button">
          <i class="fas fa-heading"></i> Heading
        </button>
        <div class="dropdown-menu">
          {#each [1, 2, 3, 4, 5, 6] as level}
            <button
              class={editor.isActive("heading", { level }) ? "is-active" : ""}
              onclick={() => setHeading(level)}
              type="button"
            >
              H{level}
            </button>
          {/each}
        </div>
      </div>
      <button
        type="button"
        aria-label="Bullet list"
        title="Bullet list"
        onclick={() => toggleChain("bulletList")}
        class={editor.isActive("bulletList") ? "is-active" : ""}
      >
        <i class="fas fa-list-ul"></i>
      </button>
      <button
        type="button"
        aria-label="Ordered list"
        title="Ordered list"
        onclick={() => toggleChain("orderedList")}
        class={editor.isActive("orderedList") ? "is-active" : ""}
      >
        <i class="fas fa-list-ol"></i>
      </button>
      <button
        type="button"
        aria-label="Code block"
        title="Code block"
        onclick={() => toggleChain("codeBlock")}
        class={editor.isActive("codeBlock") ? "is-active" : ""}
      >
        <i class="fas fa-file-code"></i>
      </button>
      <button
        type="button"
        aria-label="Blockquote"
        title="Blockquote"
        onclick={() => toggleChain("blockquote")}
        class={editor.isActive("blockquote") ? "is-active" : ""}
      >
        <i class="fas fa-quote-right"></i>
      </button>
      <button
        type="button"
        aria-label="Horizontal rule"
        title="Horizontal rule"
        onclick={() => toggleChain("horizontalRule")}
      >
        <i class="fas fa-minus"></i>
      </button>
      <button
        type="button"
        aria-label="Undo"
        title="Undo"
        onclick={() => toggleChain("undo")}
        disabled={!canToggleChain("undo")}
      >
        <i class="fas fa-undo"></i>
      </button>
      <button
        type="button"
        aria-label="Redo"
        title="Redo"
        onclick={() => toggleChain("redo")}
        disabled={!canToggleChain("redo")}
      >
        <i class="fas fa-redo"></i>
      </button>
      <button
        aria-label="Link"
        type="button"
        onclick={() => promptLink()}
        class={editor?.isActive("link") ? "is-active" : ""}
      >
        <i class="fas fa-link"></i>
      </button>
      <button aria-label="Unset link" type="button" onclick={() => unsetLink()}>
        <i class="fas fa-unlink"></i>
      </button>
      <button type="button" onclick={() => insertPlaceholder("placeholder")}>
        {"[placeholder]"}
      </button>
    </div>
  </div>
{/if}
<div class="text-editor" bind:this={element}></div>
{#if editor}
  <div class="control-group footer">
    <div class="button-group text-end">
      <button
        type="button"
        aria-label="Clear marks"
        title="Clear marks"
        onclick={() => editor.chain().focus().unsetAllMarks().run()}
      >
        <i class="fas fa-eraser"></i>
      </button>
      <button
        type="button"
        aria-label="Clear nodes"
        title="Clear nodes"
        onclick={() => editor.chain().focus().clearNodes().run()}
      >
        <i class="fas fa-ban"></i>
      </button>
    </div>
  </div>
{/if}

<style>
  .control-group.footer {
    border-radius: 0 0 8px 8px;
    border-top: none;
  }
  .control-group {
    border: 1px solid var(--border-color);
    border-radius: 8px 8px 0 0;
    background: var(--nav-bg-color);
    padding: 0.5rem 0.3rem;
    box-shadow: 0 1px 2px var(--box-shadow-color);
    user-select: none;
  }
  .dropdown {
    position: relative;
    display: inline-block;
  }
  .dropdown-toggle {
    padding: 0.5rem 1rem;
    border: 1px solid var(--border-color);
    background: var(--white);
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    box-shadow: 0 1px 2px var(--box-shadow-color);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .dropdown-menu {
    display: none;
    position: absolute;
    left: 0;
    top: 100%;
    min-width: 120px;
    background: var(--white);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    box-shadow: 0 2px 8px var(--box-shadow-color);
    z-index: 10;
    flex-direction: column;
    padding: 0.5rem 0;
  }
  .dropdown:hover .dropdown-menu {
    display: flex;
  }
  .dropdown-menu button {
    background: none;
    border: none;
    padding: 0.5rem 1rem;
    text-align: left;
    font-size: 1rem;
    cursor: pointer;
  }
  .dropdown-menu button.is-active,
  .dropdown-menu button:hover {
    background: var(--nav-hover-color);
    color: var(--primary-hover-color);
  }

  .control-group .button-group > button,
  .control-group .dropdown > button {
    padding: 0.3rem 0.7rem;
    cursor: pointer;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.03);
    border-radius: 4px;
    margin-right: 0.1px;
    margin-bottom: 0.1rem;
    color: var(--black);
    background-color: var(--nav-color);
    border: 1px solid var(--border-color);
    font-size: 0.8rem;
  }

  .control-group .button-group > button.is-active,
  .control-group .button-group > button:hover,
  .control-group .dropdown > button.is-active,
  .control-group .dropdown > button:hover {
    background-color: var(--white);
    color: var(--primary-hover-color);
  }

  .text-editor {
    border: 1px solid var(--border-color);
    border-top: none;
    border-radius: 0;
    padding: 0.5rem;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.03);
    overflow: auto;
  }

  .text-editor {
    border: 0;
    padding: 0 !important;
  }
</style>
