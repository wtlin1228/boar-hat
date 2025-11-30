import { ReactNode, useState } from "react";

import { NormalToLargeButton } from "@fe/components/button";
import {
  Dialog,
  DialogBody,
  DialogClose,
  DialogDescription,
  DialogFooter,
  DialogTitle,
} from "@fe/components/dialog";
import { Icons } from "@fe/icons/index-fixed";

export const ConfirmDeleteDialog = ({
  trigger,
  onConfirm,
}: {
  trigger: ReactNode;
  onConfirm: () => void;
}) => {
  const [open, setOpen] = useState(false);

  return (
    <Dialog size="small" trigger={trigger} open={open} onOpenChange={setOpen}>
      <DialogBody>
        <div className="flex justify-center w-full mt-8 mb-5">
          <Icons.ConfirmIcon className="w-20 h-20" />
        </div>
        <DialogTitle className="mb-4 justify-center">
          Deleting a message
        </DialogTitle>

        <DialogDescription className="px-8 text-center">
          Are you sure you want to delete this message? You won't be able to
          recover it.
        </DialogDescription>
      </DialogBody>
      <DialogFooter className="flex flex-col-reverse sm:flex-row gap-2 justify-between">
        <NormalToLargeButton appearance="text" onClick={() => setOpen(false)}>
          No, cancel
        </NormalToLargeButton>
        <NormalToLargeButton
          onClick={() => {
            setOpen(false);
            onConfirm();
          }}
        >
          Yep, do it!
        </NormalToLargeButton>
      </DialogFooter>
      <DialogClose />
    </Dialog>
  );
};
