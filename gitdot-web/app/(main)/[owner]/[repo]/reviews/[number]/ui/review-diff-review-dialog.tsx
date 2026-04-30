"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { ReviewCommentResource } from "gitdot-api";
import { useParams } from "next/navigation";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";
import { UserImage } from "../../../../ui/user-image";
import { useReviewContext } from "../context";

type ReviewVerdict = "approve" | "reject" | "comment";

export function ReviewDiffReviewDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const { review, activeDiff, activeDiffDraftComments, reviewActiveDiff } =
    useReviewContext();
  const { user } = useUserContext();
  const params = useParams<{ owner: string; repo: string }>();

  const isAuthor = user?.id === review.author?.id;
  const isMerged = activeDiff.status === "merged";
  const restrictedToComment = isAuthor || isMerged;

  const [verdict, setVerdict] = useState<ReviewVerdict | null>(
    restrictedToComment ? "comment" : null,
  );
  const [overallComment, setOverallComment] = useState("");
  const [pending, setPending] = useState(false);

  async function handleSubmit() {
    if (!verdict) return;
    setPending(true);
    const result = await reviewActiveDiff(verdict, overallComment);
    setPending(false);
    if (!("error" in result)) {
      setOpen(false);
    }
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl! top-[40%]! p-0! gap-0! flex flex-col overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Review</DialogTitle>
        </VisuallyHidden>

        <div className="flex h-32">
          <div className="flex flex-col flex-2 p-2">
            <textarea
              placeholder="Leave overall feedback..."
              value={overallComment}
              onChange={(e) => setOverallComment(e.target.value)}
              className="w-full flex-1 resize-none bg-transparent text-sm outline-none placeholder:text-muted-foreground"
            />
          </div>

          <div className="flex flex-col flex-1 h-full border-l border-border">
            {(
              [
                {
                  v: "approve",
                  label: "Approve",
                  sub: "Approve merging",
                  disabled: restrictedToComment,
                  title: restrictedToComment
                    ? isAuthor
                      ? "Authors cannot approve their own diffs"
                      : "Cannot approve a merged diff"
                    : undefined,
                },
                {
                  v: "reject",
                  label: "Reject",
                  sub: "Reject merging",
                  disabled: restrictedToComment,
                  title: restrictedToComment
                    ? isAuthor
                      ? "Authors cannot reject their own diffs"
                      : "Cannot reject a merged diff"
                    : undefined,
                },
                {
                  v: "comment",
                  label: "Comment",
                  sub: "Leave general feedback",
                  disabled: false,
                  title: undefined,
                },
              ] as const
            ).map(({ v, label, sub, disabled, title }) => (
              <button
                key={v}
                type="button"
                disabled={disabled}
                title={title}
                onClick={() =>
                  !disabled && setVerdict(verdict === v ? null : v)
                }
                className={cn(
                  "flex flex-1 items-center gap-1.5 px-3 text-left transition-colors duration-150 border-b border-border last:border-b-0",
                  disabled
                    ? "opacity-40 cursor-not-allowed"
                    : "hover:bg-accent",
                )}
              >
                <div className="flex items-start gap-1.5">
                  <div
                    className={cn(
                      "mt-[3px] shrink-0 w-3 h-3 rounded-xs border border-border transition-colors duration-150",
                      verdict === v ? "bg-foreground" : "bg-background",
                    )}
                  />
                  <div className="flex flex-col font-mono">
                    <span className="text-xs">{label}</span>
                    <span className="text-xs text-muted-foreground">{sub}</span>
                  </div>
                </div>
              </button>
            ))}
          </div>
        </div>

        {activeDiffDraftComments.length > 0 && (
          <div className="border-t border-border pb-2">
            <DraftCommentList comments={activeDiffDraftComments} />
          </div>
        )}

        <div className="flex items-center justify-between pl-2 border-t border-border h-7">
          <span className="text-xs text-muted-foreground">
            Reviewing diff #{activeDiff.position}/N in{" "}
            <span className="text-foreground">
              {params.owner}/{params.repo}
            </span>
          </span>
          <button
            type="button"
            disabled={verdict === null || pending}
            onClick={handleSubmit}
            className={cn(
              "px-3 h-7 text-xs bg-primary text-primary-foreground disabled:cursor-not-allowed enabled:hover:bg-primary/90 underline decoration-transparent enabled:hover:decoration-current transition-all duration-300",
              (verdict === null || pending) && "opacity-50",
            )}
          >
            {pending ? "Submitting..." : "Submit"}
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}

function DraftCommentList({ comments }: { comments: ReviewCommentResource[] }) {
  return (
    <div className="flex flex-col">
      <span className="text-xs text-muted-foreground font-mono px-2 py-1">
        {comments.length} draft {comments.length === 1 ? "comment" : "comments"}
      </span>
      <div className="flex flex-col gap-2 max-h-48 overflow-y-auto">
        {comments.map((comment) => (
          <DraftCommentPreview key={comment.id} comment={comment} />
        ))}
      </div>
    </div>
  );
}

function DraftCommentPreview({ comment }: { comment: ReviewCommentResource }) {
  return (
    <div className="group flex gap-2 px-2.5 py-1">
      <div className="pt-0.5 shrink-0">
        <UserImage userId={comment.author_id} px={16} />
      </div>
      <div className="flex flex-col gap-0.5 min-w-0 flex-1">
        {comment.file_path && (
          <span className="text-xs text-muted-foreground font-mono truncate">
            {comment.file_path}
            {comment.line_number_start != null && (
              <span className="text-muted-foreground/60">
                :{comment.line_number_start}
                {comment.line_number_end != null &&
                  comment.line_number_end !== comment.line_number_start &&
                  `–${comment.line_number_end}`}
              </span>
            )}
          </span>
        )}
        <span className="text-sm text-foreground line-clamp-2">
          {comment.body}
        </span>
        <div className="flex items-center gap-1.5">
          <button
            type="button"
            className="text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer"
          >
            edit
          </button>
          <button
            type="button"
            className="text-xs text-muted-foreground hover:text-destructive transition-colors cursor-pointer"
          >
            delete
          </button>
        </div>
      </div>
    </div>
  );
}
