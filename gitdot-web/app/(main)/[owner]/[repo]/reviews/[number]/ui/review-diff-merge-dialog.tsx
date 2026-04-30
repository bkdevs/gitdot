"use client";

import { useState } from "react";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { useReviewContext } from "../context";

export function ReviewDiffMergeDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const { activeDiff, mergeActiveDiff } = useReviewContext();
  const [pending, setPending] = useState(false);
  const [mergeError, setMergeError] = useState<string | null>(null);

  return (
    <Dialog
      open={open}
      onOpenChange={(o) => {
        if (!o) {
          setOpen(false);
          setMergeError(null);
        }
      }}
    >
      <DialogContent animations showOverlay className="p-0 overflow-hidden w-96">
        <div className="px-2 py-2 flex flex-col gap-0 pb-1">
          <DialogTitle className="text-sm font-normal text-foreground">
            Merge {activeDiff.message}
          </DialogTitle>
          <p className="text-xs text-muted-foreground">
            Once merged, this diff will be closed for review.
          </p>
        </div>
        {mergeError && (
          <p className="px-2 pb-1 text-xs text-red-500">{mergeError}</p>
        )}
        <div className="flex items-center justify-end h-7 border-t border-border">
          <button
            type="button"
            onClick={() => {
              setOpen(false);
              setMergeError(null);
            }}
            className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={async () => {
              setPending(true);
              const result = await mergeActiveDiff();
              setPending(false);
              if ("error" in result) {
                setMergeError(result.error);
              } else {
                setOpen(false);
              }
            }}
            disabled={pending}
            className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity cursor-pointer"
          >
            {pending ? "Merging..." : "Merge"}
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
