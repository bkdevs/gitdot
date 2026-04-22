"use client";

import type {
  ReviewCommentResource,
  ReviewDiffResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { useSearchParams } from "next/navigation";
import { createContext, useContext, useMemo, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import {
  type AddReviewerActionResult,
  addReviewerAction,
  type PublishReviewActionResult,
  publishReviewAction,
  type RemoveReviewerActionResult,
  removeReviewerAction,
  type UpdateReviewActionResult,
  updateReviewAction,
} from "@/actions/review";

export type CreateReviewCommentActionResult =
  | { comment: ReviewCommentResource }
  | { error: string };

export type {
  AddReviewerActionResult,
  PublishReviewActionResult,
  RemoveReviewerActionResult,
  UpdateReviewActionResult,
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
  draftComments: ReviewCommentResource[];
  activeComment: ReviewCommentResource | null;
  setActiveComment: (comment: ReviewCommentResource | null) => void;
  activeDiff: ReviewDiffResource;
  activeDiffComments: ReviewCommentResource[];

  publishReview: () => Promise<PublishReviewActionResult>;
  discardReview: () => Promise<{ success: true } | { error: string }>;
  addReviewer: (userName: string) => Promise<AddReviewerActionResult>;
  removeReviewer: (reviewerName: string) => Promise<RemoveReviewerActionResult>;
  addComment: (
    request: AddCommentRequest,
  ) => Promise<CreateReviewCommentActionResult>;
  updateReview: (request: {
    title?: string;
    description?: string;
  }) => Promise<UpdateReviewActionResult>;
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
  const { user } = useUserContext();
  const [review, setReview] = useState(initialReview);
  const [activeComment, setActiveComment] =
    useState<ReviewCommentResource | null>(null);
  const [draftComments, setDraftComments] = useState<ReviewCommentResource[]>(
    [],
  );
  const searchParams = useSearchParams();
  const activeDiff = useMemo(() => {
    const position = Number(searchParams.get("diff") ?? 1);
    return review.diffs.find((d) => d.position === position) ?? review.diffs[0];
  }, [searchParams, review.diffs]);

  const comments = useMemo(
    () => [...review.comments, ...draftComments],
    [review.comments, draftComments],
  );

  const activeDiffComments = useMemo(
    () => comments.filter((c) => c.diff_id === activeDiff.id),
    [comments, activeDiff.id],
  );

  async function publishReview(): Promise<PublishReviewActionResult> {
    const result = await publishReviewAction(owner, repo, review.number, {});
    if ("error" in result) return result;
    setReview(result.review);
    return result;
  }

  async function discardReview(): Promise<
    { success: true } | { error: string }
  > {
    return { error: "not implemented" };
  }

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

  async function updateReview(request: {
    title?: string;
    description?: string;
  }): Promise<UpdateReviewActionResult> {
    const result = await updateReviewAction(
      owner,
      repo,
      review.number,
      request,
    );
    if ("error" in result) return result;

    setReview((r) => ({ ...r, ...result.review }));
    return result;
  }

  function addComment(
    request: AddCommentRequest,
  ): Promise<CreateReviewCommentActionResult> {
    const latestRevision =
      activeDiff.revisions[activeDiff.revisions.length - 1];

    const now = new Date().toISOString();
    const draftComment: ReviewCommentResource = {
      id: crypto.randomUUID(),
      review_id: review.id,
      diff_id: activeDiff.id,
      revision_id: latestRevision.id,
      author_id: user?.id ?? "00000000-0000-0000-0000-000000000000",
      parent_id: null,
      body: request.body,
      file_path: request.file_path ?? null,
      line_number_start: request.line_number_start ?? null,
      line_number_end: request.line_number_end ?? null,
      start_character: request.start_character ?? null,
      end_character: request.end_character ?? null,
      side: request.side ?? null,
      resolved: false,
      created_at: now,
      updated_at: now,
      author: user ? { id: user.id, name: user.name } : null,
    };

    setDraftComments((prev) => [...prev, draftComment]);
    return Promise.resolve({ comment: draftComment });
  }

  return (
    <ReviewContext
      value={{
        review,
        diffs: review.diffs,
        reviewers: review.reviewers,
        comments,
        draftComments,
        activeComment,
        setActiveComment,
        activeDiff,
        activeDiffComments,
        publishReview,
        discardReview,
        addReviewer,
        removeReviewer,
        addComment,
        updateReview,
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
