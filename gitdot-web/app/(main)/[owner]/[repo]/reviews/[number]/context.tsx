"use client";

import type {
  ReviewCommentResource,
  ReviewDiffResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { createContext, useContext, useState } from "react";
import {
  type AddReviewerActionResult,
  addReviewerAction,
  type RemoveReviewerActionResult,
  removeReviewerAction,
} from "@/actions/review";

export type { AddReviewerActionResult, RemoveReviewerActionResult };

type ReviewContext = {
  review: ReviewResource;
  diffs: ReviewDiffResource[];
  reviewers: ReviewerResource[];
  comments: ReviewCommentResource[];

  addReviewer: (userName: string) => Promise<AddReviewerActionResult>;
  removeReviewer: (reviewerName: string) => Promise<RemoveReviewerActionResult>;
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

  return (
    <ReviewContext
      value={{
        review,
        diffs: review.diffs,
        reviewers: review.reviewers,
        comments: review.comments,
        addReviewer,
        removeReviewer,
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
