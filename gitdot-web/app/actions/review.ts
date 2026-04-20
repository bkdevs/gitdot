"use server";

import type {
  CreateReviewCommentRequest,
  JudgeReviewDiffRequest,
  PublishReviewRequest,
  ReviewCommentResource,
  ReviewerResource,
  ReviewResource,
  UpdateReviewCommentRequest,
  UpdateReviewDiffRequest,
  UpdateReviewRequest,
} from "gitdot-api";
import { refresh } from "next/cache";
import {
  ApiError,
  addReviewer,
  createReviewComment,
  mergeDiff,
  publishReview,
  removeReviewer,
  resolveReviewComment,
  submitReview,
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

export type SubmitReviewActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function judgeDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
  request: JudgeReviewDiffRequest,
): Promise<SubmitReviewActionResult> {
  const result = await submitReview(owner, repo, number, position, request);
  if (!result) {
    return { error: "submitReview call failed" };
  }

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

export type CreateReviewCommentActionResult =
  | { comment: ReviewCommentResource }
  | { error: string };

export async function createReviewCommentAction(
  owner: string,
  repo: string,
  number: number,
  request: CreateReviewCommentRequest,
): Promise<CreateReviewCommentActionResult> {
  const result = await createReviewComment(owner, repo, number, request);
  if (!result) {
    return { error: "createReviewComment call failed" };
  }

  refresh();
  return { comment: result };
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
