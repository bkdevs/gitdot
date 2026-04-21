"use client";

import type {
  ReviewCommentResource,
  ReviewDiffResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { useSearchParams } from "next/navigation";
import { createContext, useContext, useMemo, useState } from "react";
import {
  type AddReviewerActionResult,
  addReviewerAction,
  type CreateReviewCommentActionResult,
  createReviewCommentAction,
  type RemoveReviewerActionResult,
  removeReviewerAction,
} from "@/actions/review";

export type {
  AddReviewerActionResult,
  CreateReviewCommentActionResult,
  RemoveReviewerActionResult,
};

export type AddCommentRequest = {
  body: string;
  file_path?: string;
  line_number_start?: number;
  line_number_end?: number;
  start_character?: number;
  end_character?: number;
  side?: string;
};

type ReviewContext = {
  review: ReviewResource;
  diffs: ReviewDiffResource[];
  reviewers: ReviewerResource[];
  comments: ReviewCommentResource[];
  activeComment: ReviewCommentResource | null;
  setActiveComment: (comment: ReviewCommentResource | null) => void;
  activeDiff: ReviewDiffResource;
  activeDiffComments: ReviewCommentResource[];

  addReviewer: (userName: string) => Promise<AddReviewerActionResult>;
  removeReviewer: (reviewerName: string) => Promise<RemoveReviewerActionResult>;
  addComment: (
    request: AddCommentRequest,
  ) => Promise<CreateReviewCommentActionResult>;
};

const ReviewContext = createContext<ReviewContext | null>(null);

export function ReviewProvider({
  owner,
  repo,
  review: initialReview,
  children,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
  children: React.ReactNode;
}) {
  const [review, setReview] = useState(initialReview);
  const [activeComment, setActiveComment] =
    useState<ReviewCommentResource | null>(null);
  const searchParams = useSearchParams();
  const activeDiff = useMemo(() => {
    const position = Number(searchParams.get("diff") ?? 1);
    return review.diffs.find((d) => d.position === position) ?? review.diffs[0];
  }, [searchParams, review.diffs]);

  const activeDiffComments = useMemo(
    () => review.comments.filter((c) => c.diff_id === activeDiff.id),
    [review.comments, activeDiff.id],
  );

  async function addReviewer(
    userName: string,
  ): Promise<AddReviewerActionResult> {
    const result = await addReviewerAction(
      owner,
      repo,
      review.number,
      userName,
    );
    if ("error" in result) return result;

    setReview((r) => ({ ...r, reviewers: [...r.reviewers, result.reviewer] }));
    return result;
  }

  async function removeReviewer(
    reviewerName: string,
  ): Promise<RemoveReviewerActionResult> {
    const result = await removeReviewerAction(
      owner,
      repo,
      review.number,
      reviewerName,
    );
    if ("error" in result) return result;

    setReview((r) => ({
      ...r,
      reviewers: r.reviewers.filter((rv) => rv.user?.name !== reviewerName),
    }));
    return result;
  }

  async function addComment(
    request: AddCommentRequest,
  ): Promise<CreateReviewCommentActionResult> {
    const latestRevision =
      activeDiff.revisions[activeDiff.revisions.length - 1];
    const result = await createReviewCommentAction(owner, repo, review.number, {
      ...request,
      diff_id: activeDiff.id,
      revision_id: latestRevision.id,
    });
    if ("error" in result) return result;

    setReview((r) => ({ ...r, comments: [...r.comments, result.comment] }));
    return result;
  }

  return (
    <ReviewContext
      value={{
        review,
        diffs: review.diffs,
        reviewers: review.reviewers,
        comments: review.comments,
        activeComment,
        setActiveComment,
        activeDiff,
        activeDiffComments,
        addReviewer,
        removeReviewer,
        addComment,
      }}
    >
      {children}
    </ReviewContext>
  );
}

export function useReviewContext(): ReviewContext {
  const ctx = useContext(ReviewContext);
  if (!ctx)
    throw new Error("useReviewContext must be used within ReviewProvider");
  return ctx;
}
