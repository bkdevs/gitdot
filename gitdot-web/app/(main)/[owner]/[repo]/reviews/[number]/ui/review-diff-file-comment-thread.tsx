"use client";

import { useImperativeHandle, useMemo, useState } from "react";
import { timeAgo } from "@/util";
import { useReviewContext } from "../context";

export type ReviewDiffFileCommentThreadHandle = {
  open: (pos: { x: number; y: number }) => void;
  close: () => void;
};

export function ReviewDiffFileCommentThread({
  onClose,
  ref,
}: {
  onClose: () => void;
  ref: React.Ref<ReviewDiffFileCommentThreadHandle>;
}) {
  const { activeComment, activeDiffComments } = useReviewContext();
  const [open, setOpen] = useState(false);
  const [pos, setPos] = useState({ x: 0, y: 0 });

  useImperativeHandle(ref, () => ({
    open(p) {
      setPos(p);
      setOpen(true);
    },
    close() {
      setOpen(false);
    },
  }));

  const threadComments = useMemo(() => {
    if (!activeComment) return [];
    return activeDiffComments.filter(
      (c) =>
        c.file_path === activeComment.file_path &&
        c.line_number_start === activeComment.line_number_start &&
        c.side === activeComment.side,
    );
  }, [activeComment, activeDiffComments]);

  if (!open || threadComments.length === 0) return null;

  function handleClose() {
    setOpen(false);
    onClose();
  }

  return (
    <>
      <div className="fixed inset-0 z-[60]" onMouseDown={handleClose} />
      <div
        className="fixed z-[70] w-72 bg-background border border-border shadow-md overflow-hidden"
        style={{ top: pos.y + 12, left: pos.x + 12 }}
        onMouseDown={(e) => e.stopPropagation()}
      >
        {threadComments.map((comment) => (
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
