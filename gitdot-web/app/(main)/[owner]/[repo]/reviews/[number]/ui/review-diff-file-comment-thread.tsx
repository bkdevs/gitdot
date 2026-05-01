"use client";

import { Send } from "lucide-react";
import { useImperativeHandle, useMemo, useState } from "react";
import { timeAgo } from "@/util";
import { UserImage } from "../../../../ui/user-image";
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
  const { activeComment, activeDiffComments, addComment } = useReviewContext();
  const [open, setOpen] = useState(false);
  const [pos, setPos] = useState({ x: 0, y: 0 });
  const [reply, setReply] = useState("");

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

  const lastId = threadComments[threadComments.length - 1].id;

  function handleClose() {
    setOpen(false);
    onClose();
  }

  function sendReply() {
    const body = reply.trim();
    if (!body) return;
    addComment({ body, parent_id: lastId });
    setReply("");
  }

  return (
    <>
      <div className="fixed inset-0 z-[60]" onMouseDown={handleClose} />
      <div
        className="fixed z-[70] w-72 bg-background border border-border shadow-md rounded-sm overflow-hidden"
        style={{ top: pos.y + 12, left: pos.x + 12 }}
        onMouseDown={(e) => e.stopPropagation()}
      >
        <div className="flex flex-col pb-2">
          {threadComments.map((comment) => (
            <div key={comment.id} className="flex gap-2 px-2 py-1.5">
              <div className="pt-0.5 shrink-0">
                <UserImage userId={comment.author_id} px={18} />
              </div>
              <div className="flex flex-col flex-1 min-w-0">
                <div className="flex items-center gap-1.5">
                  <span className="text-xs text-foreground">
                    {comment.author?.name}
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {timeAgo(new Date(comment.created_at))}
                  </span>
                </div>
                <p className="text-sm text-foreground whitespace-pre-wrap">
                  {comment.body}
                </p>
              </div>
            </div>
          ))}
        </div>
        <div className="flex min-h-7 border-t items-start">
          <textarea
            value={reply}
            onChange={(e) => setReply(e.target.value)}
            placeholder="Reply..."
            rows={1}
            className="flex-1 min-w-0 px-2 py-1.5 text-xs bg-transparent outline-none resize-none field-sizing-content placeholder:text-muted-foreground"
            onKeyDown={(e) => {
              if (e.key === "Escape") {
                handleClose();
              } else if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                sendReply();
              }
            }}
          />
          <button
            type="button"
            className="px-2 mt-1.5 text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
            onClick={sendReply}
          >
            <Send size={14} />
          </button>
        </div>
      </div>
    </>
  );
}
