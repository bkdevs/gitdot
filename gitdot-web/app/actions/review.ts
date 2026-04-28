"use server";

import type {
  CreateReviewCommentsRequest,
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
  approveReviewDiff,
  createReviewComments,
  mergeDiff,
  mergeReview,
  publishReview,
  publishReviewDiff,
  rejectReviewDiff,
  removeReviewer,
  resolveReviewComment,
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

export type MergeReviewActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function mergeReviewAction(
  owner: string,
  repo: string,
  number: number,
): Promise<MergeReviewActionResult> {
  const result = await mergeReview(owner, repo, number);
  if (!result) {
    return { error: "mergeReview call failed" };
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

export type ApproveReviewDiffActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function approveReviewDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
): Promise<ApproveReviewDiffActionResult> {
  const result = await approveReviewDiff(owner, repo, number, position);
  if (!result) return { error: "approveReviewDiff call failed" };
  refresh();
  return { review: result };
}

export type RejectReviewDiffActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function rejectReviewDiffAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
): Promise<RejectReviewDiffActionResult> {
  const result = await rejectReviewDiff(owner, repo, number, position);
  if (!result) return { error: "rejectReviewDiff call failed" };
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

export type CreateReviewCommentsActionResult =
  | { review: ReviewResource }
  | { error: string };

export async function createReviewCommentsAction(
  owner: string,
  repo: string,
  number: number,
  position: number,
  request: CreateReviewCommentsRequest,
): Promise<CreateReviewCommentsActionResult> {
  const result = await createReviewComments(
    owner,
    repo,
    number,
    position,
    request,
  );
  if (!result) return { error: "createReviewComments call failed" };
  refresh();
  return { review: result };
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
