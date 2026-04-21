"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { useEffect, useRef } from "react";
import { cn } from "@/util";
import { UserImage } from "@/(main)/[owner]/ui/user-image";

export function ReviewDiffFileBubbles({
  commentThreads,
  userId,
  activeCommentId,
}: {
  commentThreads: Array<{ top: number; comments: ReviewCommentResource[] }>;
  userId: string | undefined;
  activeCommentId: string | null;
}) {
  if (commentThreads.length === 0) return null;

  return (
    <>
      {commentThreads.map((thread) => {
        const isActive =
          activeCommentId != null &&
          thread.comments.some((c) => c.id.startsWith(activeCommentId));
        return (
          <ReviewBubble
            key={thread.comments[0].id}
            thread={thread}
            userId={userId}
            isActive={isActive}
          />
        );
      })}
    </>
  );
}

function ReviewBubble({
  thread,
  userId,
  isActive,
}: {
  thread: { top: number; comments: ReviewCommentResource[] };
  userId: string | undefined;
  isActive: boolean;
}) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (isActive) {
      ref.current?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  }, [isActive]);

  return (
    <div
      ref={ref}
      id={`comment-${thread.comments[0].id}`}
      className={cn(
        "absolute z-50 flex flex-row items-center gap-1.5 left-full ml-2 px-2 py-0.5 bg-background border border-border rounded-full animate-in fade-in duration-200",
        isActive && "bg-accent",
      )}
      style={{ top: thread.top }}
    >
      <UserImage userId={userId} px={16} />
      <span className={cn("text-xs font-sans", isActive ? "text-foreground" : "text-muted-foreground")}>
        {thread.comments.length}
      </span>
    </div>
  );
}
