import "server-only";

import type {
  JudgeReviewDiffRequest,
  PublishReviewRequest,
  UpdateReviewDiffRequest,
  UpdateReviewRequest,
} from "gitdot-api";
import {
  GetReviewDiffResponse,
  ReviewCommentResource,
  ReviewerResource,
  ReviewResource,
} from "gitdot-api";
import { z } from "zod";
import { toQueryString } from "@/util";
import {
  authDelete,
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleEmptyResponse,
  handleResponse,
} from "./util";

export async function listReviews(
  owner: string,
  repo: string,
): Promise<ReviewResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/reviews`,
  );

  return await handleResponse(response, z.array(ReviewResource));
}

export async function getReview(
  owner: string,
  repo: string,
  number: number,
): Promise<ReviewResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}`,
  );

  return await handleResponse(response, ReviewResource);
}

export async function getReviewDiff(
  owner: string,
  repo: string,
  number: number,
  position: number,
  revision?: number,
  compareTo?: number,
): Promise<GetReviewDiffResponse | null> {
  const params: Record<string, number> = {};
  if (revision !== undefined) params.revision = revision;
  if (compareTo !== undefined) params.compare_to = compareTo;
  const query = toQueryString(params);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/diff/${position}${query ? `?${query}` : ""}`,
  );

  return await handleResponse(response, GetReviewDiffResponse);
}

export async function addReviewer(
  owner: string,
  repo: string,
  number: number,
  request: { user_name: string },
): Promise<ReviewerResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/reviewer`,
    request,
  );

  return await handleResponse(response, ReviewerResource);
}

export async function removeReviewer(
  owner: string,
  repo: string,
  number: number,
  reviewerName: string,
): Promise<void> {
  const response = await authDelete(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/reviewer/${reviewerName}`,
  );

  await handleEmptyResponse(response);
}

export async function updateDiff(
  owner: string,
  repo: string,
  number: number,
  position: number,
  request: UpdateReviewDiffRequest,
): Promise<ReviewResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/diff/${position}`,
    request,
  );

  return await handleResponse(response, ReviewResource);
}

export async function updateReview(
  owner: string,
  repo: string,
  number: number,
  request: UpdateReviewRequest,
): Promise<ReviewResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}`,
    request,
  );

  return await handleResponse(response, ReviewResource);
}

export async function publishReview(
  owner: string,
  repo: string,
  number: number,
  request: PublishReviewRequest,
): Promise<ReviewResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/publish`,
    request,
  );

  return await handleResponse(response, ReviewResource);
}

export async function submitReview(
  owner: string,
  repo: string,
  number: number,
  position: number,
  request: JudgeReviewDiffRequest,
): Promise<ReviewResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/diff/${position}/submit`,
    request,
  );

  return await handleResponse(response, ReviewResource);
}

export async function mergeDiff(
  owner: string,
  repo: string,
  number: number,
  position: number,
): Promise<ReviewResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/diff/${position}/merge`,
    {},
  );

  return await handleResponse(response, ReviewResource);
}

export async function resolveReviewComment(
  owner: string,
  repo: string,
  number: number,
  commentId: string,
  resolved: boolean,
): Promise<ReviewCommentResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/review/${number}/comment/${commentId}/resolve`,
    { resolved },
  );

  return await handleResponse(response, ReviewCommentResource);
}
