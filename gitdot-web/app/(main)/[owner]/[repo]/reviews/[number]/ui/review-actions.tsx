"use client";

import { Send, Trash2 } from "lucide-react";
import { useState } from "react";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { useReviewContext } from "../context";

export function ReviewActions() {
  const { publishReview, discardReview } = useReviewContext();
  const [publishing, setPublishing] = useState(false);
  const [publishError, setPublishError] = useState<string | null>(null);
  const [discarding, setDiscarding] = useState(false);
  const [discardError, setDiscardError] = useState<string | null>(null);

  return (
    <div className="shrink-0 flex flex-col border-t border-border">
      <button
        type="button"
        onClick={() => setPublishing(true)}
        className="flex w-full h-8 items-center justify-start gap-1.5 px-2 border-b border-border text-xs text-primary bg-accent/50 outline-none underline decoration-transparent hover:decoration-current transition-colors duration-200 cursor-pointer"
      >
        <Send className="size-3.5" />
        Publish
      </button>
      <Dialog
        open={publishing}
        onOpenChange={(open) => {
          if (!open) {
            setPublishing(false);
            setPublishError(null);
          }
        }}
      >
        <DialogContent
          animations
          showOverlay
          className="p-0 overflow-hidden w-96"
        >
          <div className="px-2 py-2 flex flex-col gap-0 pb-1">
            <DialogTitle className="text-sm font-normal text-foreground">
              Publish review
            </DialogTitle>
            <p className="text-xs text-muted-foreground">
              Publishing this review will notify all reviewers.
            </p>
          </div>
          {publishError && (
            <p className="px-2 pb-1 text-xs text-red-500">{publishError}</p>
          )}
          <div className="flex items-center justify-end h-8 border-t border-border">
            <button
              type="button"
              onClick={async () => {
                const result = await publishReview();
                if ("error" in result) {
                  setPublishError(result.error);
                } else {
                  setPublishing(false);
                  setPublishError(null);
                }
              }}
              className="flex items-center px-2 h-full text-xs text-primary bg-background hover:underline hover:bg-accent/50 border-l border-border transition-colors cursor-pointer"
            >
              Publish
            </button>
          </div>
        </DialogContent>
      </Dialog>
      <button
        type="button"
        onClick={() => setDiscarding(true)}
        className="flex w-full h-8 items-center justify-start gap-1.5 px-2 text-xs text-destructive outline-none underline decoration-transparent hover:decoration-current hover:bg-accent transition-colors duration-200 cursor-pointer"
      >
        <Trash2 className="size-3.5" />
        Discard
      </button>
      <Dialog
        open={discarding}
        onOpenChange={(open) => {
          if (!open) {
            setDiscarding(false);
            setDiscardError(null);
          }
        }}
      >
        <DialogContent
          animations
          showOverlay
          className="p-0 overflow-hidden w-96"
        >
          <div className="px-2 py-2 flex flex-col gap-0 pb-1">
            <DialogTitle className="text-sm font-normal text-foreground">
              Discard review?
            </DialogTitle>
            <p className="text-xs text-muted-foreground">
              This will permanently delete your review and all comments.
            </p>
          </div>
          {discardError && (
            <p className="px-2 pb-1 text-xs text-red-500">{discardError}</p>
          )}
          <div className="flex items-center justify-end h-8 border-t border-border">
            <button
              type="button"
              onClick={() => {
                setDiscarding(false);
                setDiscardError(null);
              }}
              className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={async () => {
                const result = await discardReview();
                if ("error" in result) {
                  setDiscardError(result.error);
                } else {
                  setDiscarding(false);
                  setDiscardError(null);
                }
              }}
              className="flex items-center px-2 h-full text-xs text-destructive bg-background hover:underline hover:bg-accent/50 border-l border-border transition-colors cursor-pointer"
            >
              Discard
            </button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
