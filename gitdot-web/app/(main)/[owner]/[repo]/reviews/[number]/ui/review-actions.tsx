"use client";

import { Ellipsis, GitMerge, Send } from "lucide-react";
import { useState } from "react";
import { useTypewriter } from "@/hooks/use-typewriter";
import { formatDate, pluralize, timeAgo } from "@/util";
import { useReviewContext } from "../context";

export function ReviewActions() {
  const { review } = useReviewContext();

  if (review.status === "draft") return <ReviewDraftActions />;
  if (review.status === "in_progress") return <ReviewOpenActions />;
  return <ReviewClosedSummary />;
}

function ReviewDraftActions() {
  const { review, diffs, publishReview } = useReviewContext();
  const [pending, setPending] = useState(false);

  const pendingCount = diffs.filter((d) => d.status === "pending").length;
  const publishable = pendingCount === 0;

  const statusText = publishable
    ? "ready to publish"
    : `${pluralize(pendingCount, "diff")} pending approval`;
  const loadingText = useTypewriter(pending ? "publishing..." : "", 40);

  return (
    <div className="shrink-0 flex border-t border-border">
      <button
        type="button"
        disabled={!publishable || pending}
        onClick={async () => {
          setPending(true);
          await publishReview();
          setPending(false);
        }}
        className="flex shrink-0 w-22 h-8 items-center justify-center gap-1.5 px-3 text-xs text-primary-foreground bg-primary outline-none disabled:opacity-50 disabled:cursor-not-allowed"
      >
        <Send className="size-3.5" />
        Publish
      </button>
      <div className="flex-1 h-8 flex items-center pl-2 border-l border-border">
        <div className="flex flex-col justify-center">
          <span className="text-xs text-muted-foreground font-mono leading-none">
            {pending && loadingText ? loadingText : statusText}
          </span>
          <span className="text-[10px] text-muted-foreground/60 font-mono leading-none">
            {pluralize(diffs.length, "diff")}
            <span className="mx-1">‧</span>last updated{" "}
            {timeAgo(new Date(review.updated_at))}
          </span>
        </div>
        <button
          type="button"
          className="ml-auto h-full px-2 text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
        >
          <Ellipsis className="size-3.5" />
        </button>
      </div>
    </div>
  );
}

function ReviewOpenActions() {
  const { diffs } = useReviewContext();
  const pendingCount = diffs.filter((d) => d.status !== "merged").length;

  return (
    <div className="shrink-0 flex border-t border-border">
      <button
        type="button"
        disabled
        className="flex shrink-0 w-22 h-8 items-center justify-center gap-1.5 px-3 text-xs text-primary-foreground bg-primary outline-none opacity-50 cursor-not-allowed"
      >
        <GitMerge className="size-3.5" />
        Merge
      </button>
      <div className="flex-1 h-8 flex items-center pl-2 border-l border-border">
        <div className="flex flex-col justify-center">
          <span className="text-xs text-muted-foreground font-mono leading-none">
            {diffs.length} diffs to merge
          </span>
          <span className="text-[10px] text-muted-foreground/60 font-mono leading-none">
            {pendingCount} pending approval
          </span>
        </div>
        <button
          type="button"
          className="ml-auto h-full px-2 text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
        >
          <Ellipsis className="size-3.5" />
        </button>
      </div>
    </div>
  );
}

function ReviewClosedSummary() {
  const { review, diffs } = useReviewContext();
  const mergedCount = diffs.filter((d) => d.status === "merged").length;

  return (
    <div className="shrink-0 flex border-t border-border h-8 items-center px-3">
      <span className="text-xs text-muted-foreground font-mono">
        {mergedCount} diffs merged on {formatDate(review.updated_at)}.
      </span>
    </div>
  );
}
