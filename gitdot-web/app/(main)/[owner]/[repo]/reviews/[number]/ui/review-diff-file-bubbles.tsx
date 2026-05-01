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
  bubblePositions: Array<{
    top: number;
    parentComment: ReviewCommentResource;
    commentCount: number;
  }>;
}) {
  const { activeComment } = useReviewContext();

  if (bubblePositions.length === 0) return null;

  return (
    <>
      {bubblePositions.map((thread) => {
        const isActive =
          activeComment != null &&
          (activeComment.id === thread.parentComment.id ||
            activeComment.parent_id === thread.parentComment.id);
        return (
          <ReviewDiffFileBubble
            key={thread.parentComment.id}
            side={side}
            parentComment={thread.parentComment}
            commentCount={thread.commentCount}
            top={thread.top}
            isActive={isActive}
          />
        );
      })}
    </>
  );
}

function ReviewDiffFileBubble({
  side,
  parentComment,
  commentCount,
  top,
  isActive,
}: {
  side: "old" | "new";
  parentComment: ReviewCommentResource;
  commentCount: number;
  top: number;
  isActive: boolean;
}) {
  const { setActiveComment } = useReviewContext();

  return (
    <div
      id={`comment-${parentComment.id}`}
      className={cn(
        "absolute z-[5] flex flex-row items-center gap-1.5 px-2 py-0.5 bg-background border border-border rounded-full animate-in fade-in duration-200 hover:bg-accent select-none",
        side === "old" ? "right-full mr-2" : "left-full ml-2",
        isActive && "bg-accent",
      )}
      style={{ top }}
      onClick={() =>
        isActive ? setActiveComment(null) : setActiveComment(parentComment)
      }
    >
      <UserImage userId={parentComment.author_id} px={16} />
      <span
        className={cn(
          "text-xs font-sans",
          isActive ? "text-foreground" : "text-muted-foreground",
        )}
      >
        {commentCount}
      </span>
    </div>
  );
}
