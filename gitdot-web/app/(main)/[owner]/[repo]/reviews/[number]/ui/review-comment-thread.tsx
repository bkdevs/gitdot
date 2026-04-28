"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { useState } from "react";
import { usePathname, useRouter } from "next/navigation";
import { cn } from "@/util";
import { useReviewContext } from "../context";
import { ReviewComment } from "./review-comment";

export function ReviewCommentThread({
  comments,
}: {
  comments: ReviewCommentResource[];
}) {
  const { activeComment, setActiveComment, diffs } = useReviewContext();
  const router = useRouter();
  const pathname = usePathname();
  const isThreadActive = comments.some((c) => c.id === activeComment?.id);

  function handleClick() {
    if (isThreadActive) {
      setActiveComment(null);
      return;
    }
    const root = comments[0];
    setActiveComment(root);
    const diff = diffs.find((d) => d.id === root.diff_id);
    if (diff) router.push(`${pathname}?diff=${diff.position}`);
  }

  const root = comments[0];
  const fileName = root.file_path?.split("/").at(-1) ?? null;
  const { line_number_start: start, line_number_end: end } = root;
  const lineLabel =
    start != null
      ? end != null && end !== start
        ? `${start}–${end}`
        : `${start}`
      : null;

  return (
    <div
      className={cn(
        "flex flex-col -mx-1.5 px-1.5 rounded transition-colors duration-200 cursor-pointer select-none",
        isThreadActive && "bg-diff-orange",
      )}
      onClick={handleClick}
    >
      {fileName && (
        <div className="pb-0.5 text-xs font-mono text-muted-foreground">
          {fileName}
          {lineLabel ? `:${lineLabel}` : ""}
        </div>
      )}
      <div className="flex flex-col gap-1">
        {comments.map((comment) => (
          <ReviewComment key={comment.id} comment={comment} />
        ))}
      </div>
      <ReplyToThread
        isActive={isThreadActive}
        parentId={comments[comments.length - 1].id}
      />
    </div>
  );
}

function ReplyToThread({
  isActive,
  parentId,
}: {
  isActive: boolean;
  parentId: string;
}) {
  const { addComment } = useReviewContext();
  const [reply, setReply] = useState("");

  return (
    <div className={cn("pl-6", !isActive && "invisible")}>
      <textarea
        key={isActive ? "active" : "inactive"}
        autoFocus={isActive}
        value={reply}
        onChange={(e) => setReply(e.target.value)}
        placeholder="reply..."
        className="text-sm text-foreground w-full resize-none bg-transparent outline-none field-sizing-content border-b border-black dark:border-white placeholder:text-muted-foreground"
        onClick={(e) => e.stopPropagation()}
        onKeyDown={(e) => {
          if (e.key === "Escape") {
            e.currentTarget.blur();
          } else if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            const body = reply.trim();
            if (!body) return;
            addComment({ body, parent_id: parentId });
            setReply("");
          }
        }}
      />
    </div>
  );
}
