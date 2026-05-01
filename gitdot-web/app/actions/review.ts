"use server";

import type {
  PublishReviewRequest,
  ReplyToReviewCommentRequest,
  ReviewCommentResource,
  ReviewerResource,
  ReviewResource,
  ReviewReviewDiffRequest,
  UpdateReviewCommentRequest,
  UpdateReviewDiffRequest,
  UpdateReviewRequest,
} from "gitdot-api";
import { refresh } from "next/cache";
import {
  ApiError,
  addReviewer,
  mergeDiff,
  publishReview,
  publishReviewDiff,
  removeReviewer,
  replyToReviewComment,
  resolveReviewComment,
  reviewReviewDiff,
  updateDiff,
  updateReview,
  updateReviewComment,
} from "@/dal";

export type AddReviewerActionResult =
  | { reviewer: ReviewerResource }
  | { error: string };

export async function addReviewerAction(
  owner: string,
  repo: string,
  number: number,
  userName: string,
): Promise<AddReviewerActionResult> {
  let result: ReviewerResource | null;
  try {
    result = await addReviewer(owner, repo, number, { user_name: userName });
  } catch (e) {
    return {
      error: e instanceof ApiError ? e.message : "addReviewer call failed",
    };
  }
  if (!result) {
    return { error: "User not found" };
  }

  refresh();
  return { reviewer: result };
}

export type RemoveReviewerActionResult = { success: true } | { error: string };

export async function removeReviewerAction(
  owner: string,
  repo: string,
  number: number,
  reviewerName: string,
): Promise<RemoveReviewerActionResult> {
  try {
    await removeReviewer(owner, repo, number, reviewerName);
  } catch {
    return { error: "removeReviewer call failed" };
  }

  refresh();
  return { success: true };
}

export type UpdateDiffActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function updateDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
  request: UpdateReviewDiffRequest,
): Promise<UpdateDiffActionResult> {
  const result = await updateDiff(owner, repo, number, position, request);
  if (!result) {
    return { error: "updateDiff call failed" };
  }

  refresh();
  return { review: result };
}

export type UpdateReviewActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function updateReviewAction(
  owner: string,
  repo: string,
  number: number,
  request: UpdateReviewRequest,
): Promise<UpdateReviewActionResult> {
  const result = await updateReview(owner, repo, number, request);
  if (!result) {
    return { error: "updateReview call failed" };
  }

  refresh();
  return { review: result };
}

export type PublishReviewActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function publishReviewAction(
  owner: string,
  repo: string,
  number: number,
  request: PublishReviewRequest,
): Promise<PublishReviewActionResult> {
  const result = await publishReview(owner, repo, number, request);
  if (!result) {
    return { error: "publishReview call failed" };
  }

  refresh();
  return { review: result };
}

export type PublishReviewDiffActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function publishReviewDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
): Promise<PublishReviewDiffActionResult> {
  const result = await publishReviewDiff(owner, repo, number, position);
  if (!result) return { error: "publishReviewDiff call failed" };
  refresh();
  return { review: result };
}

export type MergeDiffActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function mergeDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
): Promise<MergeDiffActionResult> {
  const result = await mergeDiff(owner, repo, number, position);
  if (!result) {
    return { error: "mergeDiff call failed" };
  }

  refresh();
  return { review: result };
}

export type UpdateReviewCommentActionResult =
  | { comment: ReviewCommentResource }
  | { error: string };

export async function updateReviewCommentAction(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  request: UpdateReviewCommentRequest,
): Promise<UpdateReviewCommentActionResult> {
  const result = await updateReviewComment(
    owner,
    repo,
    number,
    commentId,
    request,
  );
  if (!result) {
    return { error: "updateReviewComment call failed" };
  }

  refresh();
  return { comment: result };
}

export type ReviewReviewDiffActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function reviewReviewDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
  request: ReviewReviewDiffRequest,
): Promise<ReviewReviewDiffActionResult> {
  const result = await reviewReviewDiff(owner, repo, number, position, request);
  if (!result) return { error: "reviewReviewDiff call failed" };
  refresh();
  return { review: result };
}

export type ReplyToReviewCommentActionResult =
  | { comment: ReviewCommentResource }
  | { error: string };

export async function replyToReviewCommentAction(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  request: ReplyToReviewCommentRequest,
): Promise<ReplyToReviewCommentActionResult> {
  const result = await replyToReviewComment(
    owner,
    repo,
    number,
    commentId,
    request,
  );
  if (!result) {
    return { error: "replyToReviewComment call failed" };
  }

  refresh();
  return { comment: result };
}

export type ResolveReviewCommentActionResult =
  | { comment: ReviewCommentResource }
  | { error: string };

export async function resolveReviewCommentAction(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  resolved: boolean,
): Promise<ResolveReviewCommentActionResult> {
  const result = await resolveReviewComment(
    owner,
    repo,
    number,
    commentId,
    resolved,
  );
  if (!result) {
    return { error: "resolveReviewComment call failed" };
  }

  refresh();
  return { comment: result };
}
