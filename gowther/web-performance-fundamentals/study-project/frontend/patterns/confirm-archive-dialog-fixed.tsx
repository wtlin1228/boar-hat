import { ReactNode } from "react";

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

export const ConfirmArchiveDialog = ({ trigger }: { trigger: ReactNode }) => {
  return (
    <Dialog size="small" trigger={trigger}>
      <DialogBody>
        <div className="flex justify-center w-full mt-8 mb-5">
          <Icons.ConfirmIcon className="w-20 h-20" />
        </div>
        <DialogTitle className="mb-4 justify-center">Are you sure?</DialogTitle>

        <DialogDescription className="text-center px-8">
          Your message will be archived and you won't be able to see it anymore.
        </DialogDescription>
      </DialogBody>
      <DialogFooter className="flex flex-col-reverse sm:flex-row gap-2 justify-between">
        <NormalToLargeButton appearance="text">No, cancel</NormalToLargeButton>
        <NormalToLargeButton>Yep, do it!</NormalToLargeButton>
      </DialogFooter>
      <DialogClose />
    </Dialog>
  );
};
