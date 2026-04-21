"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { useImperativeHandle, useState } from "react";
import { timeAgo } from "@/util";

export type ReviewDiffFileCommentThreadHandle = {
  open: (
    pos: { x: number; y: number },
    comments: ReviewCommentResource[],
  ) => void;
  close: () => void;
};

export function ReviewDiffFileCommentThread({
  onClose,
  ref,
}: {
  onClose: () => void;
  ref: React.Ref<ReviewDiffFileCommentThreadHandle>;
}) {
  const [open, setOpen] = useState(false);
  const [pos, setPos] = useState({ x: 0, y: 0 });
  const [comments, setComments] = useState<ReviewCommentResource[]>([]);

  useImperativeHandle(ref, () => ({
    open(p, c) {
      setPos(p);
      setComments(c);
      setOpen(true);
    },
    close() {
      setOpen(false);
    },
  }));

  if (!open || comments.length === 0) return null;

  function handleClose() {
    setOpen(false);
    onClose();
  }

  return (
    <>
      <div className="fixed inset-0 z-40" onMouseDown={handleClose} />
      <div
        className="fixed z-50 w-72 bg-background border border-border shadow-md overflow-hidden"
        style={{ top: pos.y + 12, left: pos.x + 12 }}
        onMouseDown={(e) => e.stopPropagation()}
      >
        {comments.map((comment) => (
          <div
            key={comment.id}
            className="p-2 border-b border-border last:border-b-0"
          >
            <div className="flex items-center gap-1.5 mb-1">
              <span className="text-xs font-medium">
                {comment.author?.name}
              </span>
              <span className="text-xs text-muted-foreground">
                {timeAgo(new Date(comment.created_at))}
              </span>
            </div>
            <p className="text-xs whitespace-pre-wrap">{comment.body}</p>
          </div>
        ))}
      </div>
    </>
  );
}
