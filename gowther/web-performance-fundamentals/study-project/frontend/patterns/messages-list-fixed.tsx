import { useState, lazy, Suspense } from "react";

import { Checkbox } from "@fe/components/checkbox";
import { messages } from "@fe/data/inbox-messages";
import { Icons } from "@fe/icons/index-fixed";
import { ConfirmArchiveDialog } from "@fe/patterns/confirm-archive-dialog-fixed";
import { ConfirmDeleteDialog } from "@fe/patterns/confirm-delete-dialog-fixed";
import { merge } from "@fe/utils/merge-classnames";
import * as Toast from "@radix-ui/react-toast";
import { format } from "date-fns";

const MessageEditorLazy = lazy(async () => {
  return {
    default: (await import("@fe/patterns/message-editor-fixed")).MessageEditor,
  };
});

export const MessageListFixed = () => {
  const [hoveredMessage, setHoveredMessage] = useState<number | null>(null);
  const [clickedMessage, setClickedMessage] = useState<number | null>(null);
  const [openSnackbar, setOpenSnackbar] = useState(false);

  return (
    <div className="flex flex-col w-full">
      {clickedMessage ? (
        <Suspense
          fallback={
            <div className="w-full h-full fixed top-0 left-0 opacity-50 bg-blinkNeutral300 z-50"></div>
          }
        >
          <MessageEditorLazy
            onClose={() => {
              setClickedMessage(null);
            }}
          />
        </Suspense>
      ) : null}
      {messages.map((message) => (
        <div
          key={message.id}
          className={merge(
            `px-6 py-4 border-b border-blinkGray100 dark:border-0 last:border-b-0 hover:bg-blinkGreen50 relative`,
            message.read ? "bg-blinkGray50/70" : "bg-transparent",
          )}
          onMouseEnter={() => setHoveredMessage(message.id)}
          onMouseLeave={() => setHoveredMessage(null)}
        >
          <div className="flex gap-4 items-center">
            <Checkbox />
            <Icons.Star className="text-blinkGold400 w-8 h-8" />

            <div
              className="flex flex-col w-full cursor-pointer"
              tabIndex={1}
              onClick={() => setClickedMessage(message.id)}
            >
              <div className="flex justify-between">
                <span className="block text-base blink-text-secondary">
                  From: {message.sender}
                </span>
                <span className="text-sm blink-text-secondary">
                  {format(new Date(message.date), "MMMM do, yyyy")}
                </span>
              </div>
              <div className="text-lg">{message.subject}</div>
              <div className="text-base blink-text-subdued">
                {message.snippet}
              </div>
            </div>
            {hoveredMessage === message.id ? (
              <div className="flex gap-2 absolute top-0 right-0 p-4 bg-blinkGreen50">
                <ConfirmDeleteDialog
                  trigger={
                    <button className="text-sm bg-blinkOrange50 border-blinkOrange100 dark:text-blinkNeutral900 dark:bg-blinkOrange100 dark:border-blinkOrange100 cursor-pointer rounded py-1 px-2">
                      Delete
                    </button>
                  }
                  onConfirm={() => {
                    setOpenSnackbar(true);
                  }}
                />
                <ConfirmArchiveDialog
                  trigger={
                    <button className="text-sm bg-blinkPink50 border-blinkPink300 dark:text-blinkNeutral900 dark:bg-blinkPink200 dark:border-blinkPink300 cursor-pointer rounded py-1 px-2">
                      Archive
                    </button>
                  }
                />
                <button className="text-sm bg-blinkGreen100 border-blinkGreen500 dark:text-blinkNeutral900 dark:bg-blinkGreen400 dark:border-blinkGreen500 rounded py-1 px-2 cursor-pointer">
                  Mark as Read
                </button>
                <button className="text-sm bg-blinkBlue50 border-blinkBlue400 dark:text-blinkNeutral900 dark:bg-blinkBlue100 dark:border-blinkBlue400 rounded py-1 px-2 cursor-pointer">
                  Snooze
                </button>
              </div>
            ) : null}
          </div>
        </div>
      ))}

      <Toast.Provider swipeDirection="left" duration={3000}>
        <Toast.Root
          className="grid grid-cols-[auto_max-content] bg-blinkNeutral50 items-center gap-x-4 rounded-md bg-white p-4 shadow-[hsl(206_22%_7%_/_35%)_0px_10px_38px_-10px,_hsl(206_22%_7%_/_20%)_0px_10px_20px_-15px] [grid-template-areas:_'title_action'_'description_action'] data-[state=open]:animate-slide-in-left"
          open={openSnackbar}
          onOpenChange={() => {
            setOpenSnackbar(false);
          }}
        >
          <Toast.Title className="text-base font-medium p-2 [grid-area:_title]">
            Message deleted!
          </Toast.Title>
        </Toast.Root>
        <Toast.Viewport className="fixed bottom-4 right-4 z-50 m-0 flex w-[390px] max-w-[100vw] list-none flex-col gap-2.5 outline-none" />
      </Toast.Provider>
    </div>
  );
};
