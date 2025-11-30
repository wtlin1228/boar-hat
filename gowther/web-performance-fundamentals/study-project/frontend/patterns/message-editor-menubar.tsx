import React from "react";

import { merge } from "@fe/utils/merge-classnames";
import { useCurrentEditor } from "@tiptap/react";
import { format, add } from "date-fns";

export const MenuBar = () => {
  const { editor } = useCurrentEditor();

  if (!editor) {
    return null;
  }

  const standardButtonClass = "blink-text-primary rounded px-2 py-1";
  const activeButtonClass =
    "bg-blinkBlue500 border-blinkBlue800 dark:text-blinkNeutral900 dark:bg-blinkBlue500 dark:border-blinkBlue800";
  return (
    <div className="flex flex-wrap gap-2 mb-8">
      <button
        onClick={() => editor.chain().focus().toggleBold().run()}
        disabled={!editor.can().chain().focus().toggleBold().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("bold") ? activeButtonClass : "blink-surface-strong",
        )}
      >
        Bold
      </button>
      <button
        onClick={() => editor.chain().focus().toggleItalic().run()}
        disabled={!editor.can().chain().focus().toggleItalic().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("italic")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Italic
      </button>
      <button
        onClick={() => editor.chain().focus().toggleStrike().run()}
        disabled={!editor.can().chain().focus().toggleStrike().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("strike")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Strike
      </button>
      <button
        onClick={() => editor.chain().focus().toggleCode().run()}
        disabled={!editor.can().chain().focus().toggleCode().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("code") ? activeButtonClass : "blink-surface-strong",
        )}
      >
        Code
      </button>
      <button
        onClick={() => editor.chain().focus().unsetAllMarks().run()}
        className={standardButtonClass}
      >
        Clear marks
      </button>
      <button
        onClick={() => editor.chain().focus().clearNodes().run()}
        className={standardButtonClass}
      >
        Clear nodes
      </button>
      <button
        onClick={() => editor.chain().focus().setParagraph().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("paragraph")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Paragraph
      </button>
      <button
        onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
        className={merge(
          standardButtonClass,
          editor.isActive("heading", { level: 1 })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        H1
      </button>
      <button
        onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
        className={merge(
          standardButtonClass,
          editor.isActive("heading", { level: 2 })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        H2
      </button>
      <button
        onClick={() => editor.chain().focus().toggleHeading({ level: 3 }).run()}
        className={merge(
          standardButtonClass,
          editor.isActive("heading", { level: 3 })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        H3
      </button>
      <button
        onClick={() => editor.chain().focus().toggleHeading({ level: 4 }).run()}
        className={merge(
          standardButtonClass,
          editor.isActive("heading", { level: 4 })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        H4
      </button>
      <button
        onClick={() => editor.chain().focus().toggleHeading({ level: 5 }).run()}
        className={merge(
          standardButtonClass,
          editor.isActive("heading", { level: 5 })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        H5
      </button>
      <button
        onClick={() => editor.chain().focus().toggleHeading({ level: 6 }).run()}
        className={merge(
          standardButtonClass,
          editor.isActive("heading", { level: 6 })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        H6
      </button>
      <button
        onClick={() => editor.chain().focus().toggleBulletList().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("bulletList")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Bullet list
      </button>
      <button
        onClick={() => editor.chain().focus().toggleOrderedList().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("orderedList")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Ordered list
      </button>
      <button
        onClick={() => editor.chain().focus().toggleCodeBlock().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("codeBlock")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Code block
      </button>
      <button
        onClick={() => editor.chain().focus().toggleBlockquote().run()}
        className={merge(
          standardButtonClass,
          editor.isActive("blockquote")
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Blockquote
      </button>
      <button
        onClick={() => editor.chain().focus().setHorizontalRule().run()}
        className={standardButtonClass}
      >
        Horizontal rule
      </button>
      <button
        onClick={() => editor.chain().focus().setHardBreak().run()}
        className={standardButtonClass}
      >
        Hard break
      </button>
      <button
        onClick={() => editor.chain().focus().undo().run()}
        disabled={!editor.can().chain().focus().undo().run()}
        className={standardButtonClass}
      >
        Undo
      </button>
      <button
        onClick={() => editor.chain().focus().redo().run()}
        disabled={!editor.can().chain().focus().redo().run()}
        className={standardButtonClass}
      >
        Redo
      </button>
      <button
        onClick={() => editor.chain().focus().setColor("#958DF1").run()}
        className={merge(
          standardButtonClass,
          editor.isActive("textStyle", { color: "#958DF1" })
            ? activeButtonClass
            : "blink-surface-strong",
        )}
      >
        Purple
      </button>
      <button
        onClick={() =>
          editor
            .chain()
            .focus()
            .insertContent(
              format(
                add(new Date(), {
                  days: 1,
                }),
                "MMMM dd, yyyy",
              ),
            )
            .run()
        }
        className={standardButtonClass}
      >
        Tomorrow
      </button>
    </div>
  );
};
