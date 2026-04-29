"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { cn } from "@/util";
import { useReviewContext } from "../context";

export function ReviewDiffFileBubbles({
  side,
  bubblePositions,
}: {
  side: "old" | "new";
  bubblePositions: Array<{ top: number; comments: ReviewCommentResource[] }>;
}) {
  const { activeComment } = useReviewContext();

  if (bubblePositions.length === 0) return null;

  return (
    <>
      {bubblePositions.map((thread) => {
        const isActive =
          activeComment != null &&
          thread.comments.some((c) => c.id === activeComment.id);
        return (
          <ReviewDiffFileBubble
            key={thread.comments[0].id}
            side={side}
            thread={thread}
            isActive={isActive}
          />
        );
      })}
    </>
  );
}

function ReviewDiffFileBubble({
  side,
  thread,
  isActive,
}: {
  side: "old" | "new";
  thread: { top: number; comments: ReviewCommentResource[] };
  isActive: boolean;
}) {
  const { setActiveComment } = useReviewContext();

  return (
    <div
      id={`comment-${thread.comments[0].id}`}
      className={cn(
        "absolute z-50 flex flex-row items-center gap-1.5 px-2 py-0.5 bg-background border border-border rounded-full animate-in fade-in duration-200 hover:bg-accent select-none",
        side === "old" ? "right-full mr-2" : "left-full ml-2",
        isActive && "bg-accent",
      )}
      style={{ top: thread.top }}
      onClick={() =>
        isActive ? setActiveComment(null) : setActiveComment(thread.comments[0])
      }
    >
      <UserImage userId={thread.comments[0].author_id} px={16} />
      <span
        className={cn(
          "text-xs font-sans",
          isActive ? "text-foreground" : "text-muted-foreground",
        )}
      >
        {thread.comments.length}
      </span>
    </div>
  );
}
