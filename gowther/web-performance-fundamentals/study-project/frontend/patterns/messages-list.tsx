import { useState } from "react";

import { Checkbox } from "@fe/components/checkbox";
import { messages } from "@fe/data/inbox-messages";
import { Icons } from "@fe/icons";
import { ConfirmArchiveDialog } from "@fe/patterns/confirm-archive-dialog";
import { ConfirmDeleteDialog } from "@fe/patterns/confirm-delete-dialog";
import { MessageEditor } from "@fe/patterns/message-editor";
import { merge } from "@fe/utils/merge-classnames";
import { StudyUi } from "@fe/utils/ui-wrappers";
import moment from "moment";

export const MessageList = () => {
  const [hoveredMessage, setHoveredMessage] = useState<number | null>(null);
  const [clickedMessage, setClickedMessage] = useState<number | null>(null);
  const [openSnackbar, setOpenSnackbar] = useState(false);

  return (
    <div className="flex flex-col w-full">
      {clickedMessage ? (
        <MessageEditor
          onClose={() => {
            setClickedMessage(null);
          }}
        />
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
                  {moment(message.date).format("MMMM Do, YYYY")}
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

      <StudyUi.Library.Snackbar
        open={openSnackbar}
        onClose={() => setOpenSnackbar(false)}
        message="Messsage deleted!"
      />
    </div>
  );
};
