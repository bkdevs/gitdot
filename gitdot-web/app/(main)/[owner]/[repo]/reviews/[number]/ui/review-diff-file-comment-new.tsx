"use client";

import { useCallback, useImperativeHandle, useState } from "react";
import { useReviewContext } from "../context";

export type ReviewDiffFileCommentNewHandle = {
  open: (pos: { x: number; y: number }) => void;
};

export function ReviewDiffFileCommentNew({
  diffId,
  revisionId,
  startCharacter,
  endCharacter,
  onClose,
  ref,
}: {
  diffId: string;
  revisionId: string;
  startCharacter?: number;
  endCharacter?: number;
  onClose: () => void;
  ref: React.Ref<ReviewDiffFileCommentNewHandle>;
}) {
  const { addComment } = useReviewContext();
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
              addComment({
                diff_id: diffId,
                revision_id: revisionId,
                body: comment.trim(),
                start_character: startCharacter,
                end_character: endCharacter,
              });
              close();
            }
            if (e.key === "Escape") close();
          }}
        />
      </div>
    </>
  );
}
