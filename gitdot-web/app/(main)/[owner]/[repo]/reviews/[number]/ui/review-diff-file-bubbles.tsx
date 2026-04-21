"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { useEffect, useRef } from "react";
import { usePathname, useRouter } from "next/navigation";
import { cn } from "@/util";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { useReviewContext } from "../context";

export function ReviewDiffFileBubbles({
  commentThreads,
  userId,
  activeComment,
}: {
  commentThreads: Array<{ top: number; comments: ReviewCommentResource[] }>;
  userId: string | undefined;
  activeComment: ReviewCommentResource | null;
}) {
  if (commentThreads.length === 0) return null;

  return (
    <>
      {commentThreads.map((thread) => {
        const isActive =
          activeComment != null &&
          thread.comments.some((c) => c.id === activeComment.id);
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
  const { diffs } = useReviewContext();
  const router = useRouter();
  const pathname = usePathname();

  useEffect(() => {
    if (isActive) {
      ref.current?.scrollIntoView({ behavior: "instant", block: "center" });
    }
  }, [isActive]);

  function handleClick() {
    const comment = thread.comments[0];
    const diff = diffs.find((d) => d.id === comment.diff_id);
    if (!diff) return;
    router.push(`${pathname}?diff=${diff.position}&comment=${comment.id.slice(0, 8)}`);
  }

  return (
    <div
      ref={ref}
      id={`comment-${thread.comments[0].id}`}
      className={cn(
        "absolute z-50 flex flex-row items-center gap-1.5 left-full ml-2 px-2 py-0.5 bg-background border border-border rounded-full animate-in fade-in duration-200 hover:bg-accent",
        isActive && "bg-accent",
      )}
      style={{ top: thread.top }}
      onClick={handleClick}
    >
      <UserImage userId={userId} px={16} />
      <span className={cn("text-xs font-sans", isActive ? "text-foreground" : "text-muted-foreground")}>
        {thread.comments.length}
      </span>
    </div>
  );
}
