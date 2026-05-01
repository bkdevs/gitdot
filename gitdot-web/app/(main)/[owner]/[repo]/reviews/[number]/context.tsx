"use client";

import type {
  ReviewCommentResource,
  ReviewDiffResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { useSearchParams } from "next/navigation";
import { createContext, useContext, useMemo, useState } from "react";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import {
  type AddReviewerActionResult,
  addReviewerAction,
  type MergeDiffActionResult,
  mergeDiffAction,
  type PublishReviewActionResult,
  type PublishReviewDiffActionResult,
  publishReviewAction,
  publishReviewDiffAction,
  type RemoveReviewerActionResult,
  type ReplyToReviewCommentActionResult,
  type ReviewReviewDiffActionResult,
  removeReviewerAction,
  replyToReviewCommentAction,
  reviewReviewDiffAction,
  type UpdateReviewActionResult,
  type UpdateReviewCommentActionResult,
  updateReviewAction,
  updateReviewCommentAction,
} from "@/actions/review";

export type CreateReviewCommentActionResult =
  | { comment: ReviewCommentResource }
  | { error: string };

export type {
  AddReviewerActionResult,
  MergeDiffActionResult,
  PublishReviewActionResult,
  PublishReviewDiffActionResult,
  RemoveReviewerActionResult,
  ReplyToReviewCommentActionResult,
  ReviewReviewDiffActionResult,
  UpdateReviewActionResult,
};

export type AddCommentRequest = {
  body: string;
  parent_id?: string;
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

  allComments: ReviewCommentResource[];
  draftComments: ReviewCommentResource[];
  activeComment: ReviewCommentResource | null;
  setActiveComment: (comment: ReviewCommentResource | null) => void;
  activeDiff: ReviewDiffResource;
  activeDiffComments: ReviewCommentResource[];
  activeDiffDraftComments: ReviewCommentResource[];

  publishReview: () => Promise<PublishReviewActionResult>;
  publishActiveDiff: () => Promise<PublishReviewDiffActionResult>;
  discardReview: () => Promise<{ success: true } | { error: string }>;
  addReviewer: (userName: string) => Promise<AddReviewerActionResult>;
  removeReviewer: (reviewerName: string) => Promise<RemoveReviewerActionResult>;
  addComment: (
    request: AddCommentRequest,
  ) => Promise<CreateReviewCommentActionResult>;
  replyToComment: (
    parentId: string,
    body: string,
  ) => Promise<ReplyToReviewCommentActionResult>;
  deleteDraftComment: (id: string) => void;
  updateDraftComment: (id: string, body: string) => void;
  updateComment: (
    id: string,
    body: string,
  ) => Promise<UpdateReviewCommentActionResult>;
  updateReview: (request: {
    title?: string;
    description?: string;
  }) => Promise<UpdateReviewActionResult>;

  reviewActiveDiff: (
    action: "approve" | "reject" | "comment",
    comments: {
      revision_id: string;
      body: string;
      file_path?: string;
      line_number_start?: number;
      line_number_end?: number;
      start_character?: number;
      end_character?: number;
      side?: string;
      parent_id?: string;
    }[],
  ) => Promise<ReviewReviewDiffActionResult>;
  mergeActiveDiff: () => Promise<MergeDiffActionResult>;
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

  const activeDiffDraftComments = useMemo(
    () => draftComments.filter((c) => c.diff_id === activeDiff.id),
    [draftComments, activeDiff.id],
  );

  async function publishReview(): Promise<PublishReviewActionResult> {
    const [result] = await Promise.all([
      publishReviewAction(owner, repo, review.number, {}),
      new Promise((r) => setTimeout(r, 1200)),
    ]);
    if ("error" in result) return result;
    setReview(result.review);
    toast.success("Review published");
    return result;
  }

  async function publishActiveDiff(): Promise<PublishReviewDiffActionResult> {
    const [result] = await Promise.all([
      publishReviewDiffAction(owner, repo, review.number, activeDiff.position),
      new Promise((r) => setTimeout(r, 1200)),
    ]);
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
      parent_id: request.parent_id ?? null,
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

  async function replyToComment(
    parentId: string,
    body: string,
  ): Promise<ReplyToReviewCommentActionResult> {
    const result = await replyToReviewCommentAction(
      owner,
      repo,
      review.number,
      parentId,
      { body },
    );
    if ("error" in result) return result;
    setReview((r) => ({ ...r, comments: [...r.comments, result.comment] }));
    return result;
  }

  function deleteDraftComment(id: string) {
    setDraftComments((prev) => prev.filter((c) => c.id !== id));
  }

  function updateDraftComment(id: string, body: string) {
    setDraftComments((prev) =>
      prev.map((c) => (c.id === id ? { ...c, body } : c)),
    );
  }

  async function updateComment(
    id: string,
    body: string,
  ): Promise<UpdateReviewCommentActionResult> {
    const result = await updateReviewCommentAction(
      owner,
      repo,
      review.number,
      id,
      { body },
    );
    if ("error" in result) return result;
    setReview((r) => ({
      ...r,
      comments: r.comments.map((c) => (c.id === id ? result.comment : c)),
    }));
    return result;
  }

  async function reviewActiveDiff(
    action: "approve" | "reject" | "comment",
    comments: {
      revision_id: string;
      body: string;
      file_path?: string;
      line_number_start?: number;
      line_number_end?: number;
      start_character?: number;
      end_character?: number;
      side?: string;
      parent_id?: string;
    }[],
  ): Promise<ReviewReviewDiffActionResult> {
    const [result] = await Promise.all([
      reviewReviewDiffAction(owner, repo, review.number, activeDiff.position, {
        action,
        comments,
      }),
      new Promise((r) => setTimeout(r, 1200)),
    ]);
    if ("error" in result) return result;
    setDraftComments((prev) => prev.filter((c) => c.diff_id !== activeDiff.id));
    setReview(result.review);
    toast.success(`Diff ${activeDiff.position}. reviewed`);
    return result;
  }

  async function mergeActiveDiff(): Promise<MergeDiffActionResult> {
    const [result] = await Promise.all([
      mergeDiffAction(owner, repo, review.number, activeDiff.position),
      new Promise((r) => setTimeout(r, 1200)),
    ]);
    if ("error" in result) return result;
    setReview(result.review);
    toast.success(`Diff ${activeDiff.position}. merged`);
    return result;
  }

  return (
    <ReviewContext
      value={{
        review,
        diffs: review.diffs,
        reviewers: review.reviewers,

        allComments: comments,
        draftComments,
        activeComment,
        setActiveComment,
        activeDiff,
        activeDiffComments,
        activeDiffDraftComments,

        publishReview,
        publishActiveDiff,
        discardReview,
        addReviewer,
        removeReviewer,
        addComment,
        replyToComment,
        deleteDraftComment,
        updateDraftComment,
        updateComment,
        updateReview,

        reviewActiveDiff,
        mergeActiveDiff,
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
