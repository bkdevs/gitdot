"use client";

import { Ellipsis, GitMerge } from "lucide-react";
import { useReviewContext } from "../context";

export function ReviewActions() {
  const { diffs, allComments } = useReviewContext();

  return (
    <div className="shrink-0 flex border-t border-border">
      <button
        type="button"
        disabled
        className="flex w-1/3 h-8 items-center justify-center gap-1.5 px-3 text-xs text-primary-foreground bg-primary outline-none opacity-50 cursor-not-allowed"
      >
        <GitMerge className="size-3.5" />
        Merge all
      </button>
      <div className="w-2/3 h-8 flex items-center pl-2 border-l border-border">
        <div className="flex flex-col justify-center">
          <span className="text-xs text-muted-foreground font-mono leading-none">
            {diffs.length} diffs to merge into main
          </span>
          <span className="text-[10px] text-muted-foreground/60 font-mono leading-none">
            {allComments.length} comments
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
