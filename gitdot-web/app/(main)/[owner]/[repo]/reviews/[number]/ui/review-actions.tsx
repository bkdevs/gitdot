"use client";

import type { ReviewResource } from "gitdot-api";
import { Edit2, Send, X } from "lucide-react";
import { useState } from "react";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { useReviewContext } from "../context";

export function ReviewActions({ review }: { review: ReviewResource }) {
  if (review.status !== "draft") return null;

  return (
    <div className="shrink-0 border-t border-border">
      <PublishRow />
      <EditRow />
      <CloseRow />
    </div>
  );
}

function PublishRow() {
  const { diffs, publishReview } = useReviewContext();
  const [confirming, setConfirming] = useState(false);
  const [pending, setPending] = useState(false);
  const [publishError, setPublishError] = useState<string | null>(null);
  const pendingCount = diffs.filter((d) => d.status === "draft").length;
  const publishable = pendingCount === 0;

  return (
    <>
      <button
        type="button"
        disabled={!publishable || pending}
        onClick={() => setConfirming(true)}
        className="flex w-full items-center gap-2 px-2 h-8 bg-primary text-primary-foreground text-xs disabled:opacity-60 disabled:cursor-not-allowed cursor-pointer hover:opacity-90 transition-opacity"
      >
        <Send className="size-3.5 shrink-0" />
        {pending ? "Publishing..." : "Publish"}
        {!publishable && (
          <span className="ml-auto opacity-80">
            {pendingCount} {pendingCount === 1 ? "diff" : "diffs"} pending
          </span>
        )}
      </button>
      <Dialog
        open={confirming}
        onOpenChange={(open) => {
          if (!open) {
            setConfirming(false);
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
              Publishing will notify all listed reviewers.
            </p>
          </div>
          {publishError && (
            <p className="px-2 pb-1 text-xs text-red-500">{publishError}</p>
          )}
          <div className="flex items-center justify-end h-8 border-t border-border">
            <button
              type="button"
              onClick={() => {
                setConfirming(false);
                setPublishError(null);
              }}
              className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={async () => {
                setPending(true);
                const result = await publishReview();
                setPending(false);
                if ("error" in result) {
                  setPublishError(result.error);
                } else {
                  setConfirming(false);
                }
              }}
              className={`flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary hover:opacity-90 transition-opacity cursor-pointer ${pending ? "opacity-60" : ""}`}
            >
              {pending ? "Publishing..." : "Publish"}
            </button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}

function EditRow() {
  return (
    <button
      type="button"
      className="flex w-full items-center gap-2 px-2 h-8 text-xs text-foreground cursor-pointer hover:bg-muted/50 transition-colors border-t border-border"
    >
      <Edit2 className="size-3.5 shrink-0" />
      Edit
    </button>
  );
}

function CloseRow() {
  const { discardReview } = useReviewContext();
  const [pending, setPending] = useState(false);

  return (
    <button
      type="button"
      disabled={pending}
      onClick={async () => {
        setPending(true);
        await discardReview();
        setPending(false);
      }}
      className="flex w-full items-center gap-2 px-2 h-8 text-xs text-destructive cursor-pointer hover:bg-muted/50 transition-colors border-t border-border disabled:opacity-60 disabled:cursor-not-allowed"
    >
      <X className="size-3.5 shrink-0" />
      {pending ? "Closing..." : "Close"}
    </button>
  );
}
