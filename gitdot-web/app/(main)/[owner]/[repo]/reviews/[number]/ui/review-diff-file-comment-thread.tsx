"use client";

import type { ReviewCommentResource } from "gitdot-api";
import { Send } from "lucide-react";
import { useEffect, useImperativeHandle, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
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
  const { activeCommentThread, replyToComment } = useReviewContext();
  const { user } = useUserContext();
  const [open, setOpen] = useState(false);
  const [pos, setPos] = useState({ x: 0, y: 0 });
  const [reply, setReply] = useState("");
  const [threadComments, setThreadComments] = useState<ReviewCommentResource[]>(
    [],
  );

  useEffect(() => {
    setThreadComments(activeCommentThread);
  }, [activeCommentThread]);

  useImperativeHandle(ref, () => ({
    open(p) {
      setPos(p);
      setOpen(true);
    },
    close() {
      setOpen(false);
    },
  }));

  if (!open || threadComments.length === 0) return null;

  const lastId = threadComments[threadComments.length - 1].id;

  function handleClose() {
    setOpen(false);
    onClose();
  }

  function sendReply() {
    const body = reply.trim();
    if (!body) return;

    const now = new Date().toISOString();
    const optimistic: ReviewCommentResource = {
      id: crypto.randomUUID(),
      review_id: threadComments[0].review_id,
      diff_id: threadComments[0].diff_id,
      revision_id: threadComments[0].revision_id,
      author_id: user?.id ?? "00000000-0000-0000-0000-000000000000",
      parent_id: lastId,
      body,
      file_path: threadComments[0].file_path,
      line_number_start: threadComments[0].line_number_start,
      line_number_end: threadComments[0].line_number_end,
      start_character: threadComments[0].start_character,
      end_character: threadComments[0].end_character,
      side: threadComments[0].side,
      resolved: false,
      created_at: now,
      updated_at: now,
      author: user ? { id: user.id, name: user.name } : null,
    };

    setThreadComments((prev) => [...prev, optimistic]);
    setReply("");
    replyToComment(lastId, body);
  }

  return (
    <>
      <div className="fixed inset-0 z-[60]" onMouseDown={handleClose} />
      <div
        className="fixed z-[70] w-80 bg-background border border-border shadow-md overflow-hidden"
        style={{ top: pos.y + 12, left: pos.x + 12 }}
        onMouseDown={(e) => e.stopPropagation()}
      >
        <div className="flex flex-col p-1">
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
            autoFocus
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
            className={`px-2 mt-1.5 transition-colors ${reply.trim() ? "text-foreground cursor-pointer" : "text-muted-foreground cursor-default"}`}
            onClick={sendReply}
          >
            <Send size={14} />
          </button>
        </div>
      </div>
    </>
  );
}
