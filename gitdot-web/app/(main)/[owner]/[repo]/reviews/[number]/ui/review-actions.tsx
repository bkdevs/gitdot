"use client";

import type { ReviewResource } from "gitdot-api";
import { Edit2, Send, X } from "lucide-react";
import { useState } from "react";
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
  const [pending, setPending] = useState(false);
  const pendingCount = diffs.filter((d) => d.status === "draft").length;
  const publishable = pendingCount === 0;

  return (
    <button
      type="button"
      disabled={!publishable || pending}
      onClick={async () => {
        setPending(true);
        await publishReview();
        setPending(false);
      }}
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
