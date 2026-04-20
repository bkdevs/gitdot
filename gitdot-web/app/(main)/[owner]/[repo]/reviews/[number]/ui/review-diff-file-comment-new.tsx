"use client";

import { useCallback, useImperativeHandle, useState } from "react";

export const COMMENT_WIDGET_HEIGHT = 96; // min-h-16 (64) + p-2 padding (16) + border/shadow

export type ReviewDiffFileCommentNewHandle = {
  open: (pos: { x: number; y: number }) => void;
};

export function ReviewDiffFileCommentNew({
  onAddComment,
  onClose,
  ref,
}: {
  onAddComment: (body: string) => void;
  onClose: () => void;
  ref: React.Ref<ReviewDiffFileCommentNewHandle>;
}) {
  const [open, setOpen] = useState(false);
  const [comment, setComment] = useState("");
  const [pos, setPos] = useState({ x: 0, y: 0 });

  useImperativeHandle(ref, () => ({
    open(p) {
      setPos(p);
      setOpen(true);
    },
  }));

  const close = useCallback(() => {
    setOpen(false);
    setComment("");
    onClose();
  }, [onClose]);

  if (!open) return null;

  return (
    <>
      <div className="fixed inset-0 z-40" onMouseDown={close} />
      <div
        className="fixed z-50 w-64 bg-background border border-border shadow-md overflow-hidden"
        style={{ top: pos.y + 12, left: pos.x + 12 }}
        onMouseDown={(e) => e.stopPropagation()}
      >
        <textarea
          autoFocus
          className="w-full p-2 text-xs bg-background outline-none resize-none min-h-16"
          placeholder="Leave a comment…"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && !e.shiftKey && comment.trim()) {
              e.preventDefault();
              onAddComment(comment.trim());
              close();
            }
            if (e.key === "Escape") close();
          }}
        />
      </div>
    </>
  );
}
