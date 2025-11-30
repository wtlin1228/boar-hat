import React from "react";

import { MenuBar } from "@fe/patterns/message-editor-menubar";
import { merge } from "@fe/utils/merge-classnames";
import * as DialogPrimitives from "@radix-ui/react-dialog";
import { Color } from "@tiptap/extension-color";
import ListItem from "@tiptap/extension-list-item";
import TextStyle from "@tiptap/extension-text-style";
import { EditorProvider } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import { DateTime } from "luxon";

const extensions = [
  Color.configure({ types: [TextStyle.name, ListItem.name] }),
  TextStyle.configure({
    HTMLAttributes: {
      class: "text-blinkNeutral900",
    },
  }),
  StarterKit.configure({
    bulletList: {
      keepMarks: true,
      keepAttributes: false, // TODO : Making this as `false` becase marks are not preserved when I try to preserve attrs, awaiting a bit of help
    },
    orderedList: {
      keepMarks: true,
      keepAttributes: false, // TODO : Making this as `false` becase marks are not preserved when I try to preserve attrs, awaiting a bit of help
    },
  }),
];

const content = "<p>Hello World!</p>";

export const MessageEditor = ({ onClose }: { onClose: () => void }) => {
  // Assume this comes from the server
  const timestampDate = new Date().getTime();
  const formattedDate =
    DateTime.fromMillis(timestampDate).toFormat("MMMM dd, yyyy");

  return (
    <DialogPrimitives.Root open onOpenChange={onClose}>
      <DialogPrimitives.Portal>
        <DialogPrimitives.Overlay className="fixed bg-buGray900 opacity-5 inset-0" />

        <DialogPrimitives.Content
          className={merge(
            "fixed top-0 bg-blinkNeutral50 dark:bg-blinkNeutral800 h-full overflow-y-auto shadow-md",
            "right-0 w-1/2 max-w-[90%] animate-slide-in-right",
          )}
        >
          <div className="px-8 pt-8">Last updated: {formattedDate}</div>
          <div className="tiptap p-8">
            <EditorProvider
              slotBefore={<MenuBar />}
              extensions={extensions}
              content={content}
            ></EditorProvider>
          </div>
        </DialogPrimitives.Content>
      </DialogPrimitives.Portal>
    </DialogPrimitives.Root>
  );
};
