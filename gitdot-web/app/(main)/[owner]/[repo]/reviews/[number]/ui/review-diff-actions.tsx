"use client";

import type {
  DiffStatus,
  ReviewCommentResource,
  RevisionResource,
} from "gitdot-api";
import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { useReviewContext } from "../context";

export function ReviewDiffActions({
  position,
  status,
  revision,
}: {
  position: number;
  status: DiffStatus;
  revision: RevisionResource | undefined;
}) {
  const { review, reviewDiff, activeDiffDraftComments } = useReviewContext();
  const { user } = useUserContext();

  if (status === "merged" || review.status === "closed") return null;

  const isAuthor = user?.id === review.author?.id;
  const isReviewer = review.reviewers.some((r) => r.user?.id === user?.id);

  if (isAuthor) {
    return (
      <AuthorActions
        position={position}
        isDraft={review.status === "draft"}
        approved={status === "open"}
        activeDiffDraftComments={activeDiffDraftComments}
        reviewDiff={reviewDiff}
      />
    );
  }

  if (isReviewer && review.status === "open") {
    return (
      <ReviewerActions
        position={position}
        approved={
          revision?.verdicts.some((v) => v.reviewer_id === user?.id) ?? false
        }
        activeDiffDraftComments={activeDiffDraftComments}
        reviewDiff={reviewDiff}
      />
    );
  }

  return null;
}

function AuthorActions({
  position,
  isDraft,
  approved,
  activeDiffDraftComments,
  reviewDiff,
}: {
  position: number;
  isDraft: boolean;
  approved: boolean;
  activeDiffDraftComments: ReviewCommentResource[];
  reviewDiff: (
    position: number,
    action: "comment" | "approve" | "request_changes",
  ) => Promise<unknown>;
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
      {isDraft ? (
        <>
          <ApproveButton
            approved={approved}
            onApprove={async () => {
              await reviewDiff(position, "approve");
            }}
          />
          <ReviewButton
            position={position}
            activeDiffDraftComments={activeDiffDraftComments}
            reviewDiff={reviewDiff}
          />
        </>
      ) : (
        <CommentButton />
      )}
    </div>
  );
}

function ReviewerActions({
  position,
  approved,
  activeDiffDraftComments,
  reviewDiff,
}: {
  position: number;
  approved: boolean;
  activeDiffDraftComments: ReviewCommentResource[];
  reviewDiff: (
    position: number,
    action: "comment" | "approve" | "request_changes",
  ) => Promise<unknown>;
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
      <ApproveButton
        approved={approved}
        onApprove={async () => {
          await reviewDiff(position, "approve");
        }}
      />
      <ReviewButton
        position={position}
        activeDiffDraftComments={activeDiffDraftComments}
        reviewDiff={reviewDiff}
      />
    </div>
  );
}

function ApproveButton({
  approved,
  onApprove,
}: {
  approved: boolean;
  onApprove: () => Promise<void>;
}) {
  const [isPending, setIsPending] = useState(false);

  return (
    <button
      type="button"
      disabled={approved || isPending}
      onClick={async () => {
        setIsPending(true);
        await onApprove();
        setIsPending(false);
      }}
      className={cn(
        "text-xs font-mono px-2.5 py-1 underline decoration-transparent hover:decoration-current rounded-xs border border-primary w-full disabled:cursor-not-allowed bg-primary text-primary-foreground",
        approved || isPending ? "opacity-50" : "hover:bg-primary/90",
      )}
    >
      {approved ? "Approved" : "Approve"}
    </button>
  );
}

function CommentButton() {
  return (
    <button
      type="button"
      className="text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background hover:bg-accent w-full transition-all duration-200"
    >
      Comment
    </button>
  );
}

function ReviewButton({
  position,
  activeDiffDraftComments,
  reviewDiff,
}: {
  position: number;
  activeDiffDraftComments: ReviewCommentResource[];
  reviewDiff: (
    position: number,
    action: "comment" | "approve" | "request_changes",
  ) => Promise<unknown>;
}) {
  const [open, setOpen] = useState(false);
  const [isPending, setIsPending] = useState(false);

  async function handleAction(action: "comment" | "request_changes") {
    setOpen(false);
    setIsPending(true);
    await reviewDiff(position, action);
    setIsPending(false);
  }

  return (
    <DropdownMenu open={open} onOpenChange={setOpen}>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className={cn(
            "text-xs font-mono px-2.5 py-1 rounded-xs border border-border bg-background hover:bg-accent w-full transition-all duration-200 flex items-center justify-center gap-1",
            (open || isPending) && "opacity-50",
          )}
        >
          <span>Review</span>
          {activeDiffDraftComments.length > 0 && (
            <span className="font-mono text-xs text-muted-foreground">
              ({activeDiffDraftComments.length})
            </span>
          )}
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        align="start"
        style={{ width: "var(--radix-popper-anchor-width)" }}
      >
        <DropdownMenuItem
          className="text-xs"
          onClick={() => handleAction("request_changes")}
        >
          Request changes
        </DropdownMenuItem>
        <DropdownMenuItem
          className="text-xs"
          onClick={() => handleAction("comment")}
        >
          Publish comments
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
