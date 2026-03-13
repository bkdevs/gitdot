import "server-only";

import { ReviewerResource, ReviewResource } from "gitdot-api";
import { z } from "zod";
import {
  authDelete,
  authFetch,
  authPost,
  GITDOT_SERVER_URL,
  handleEmptyResponse,
  handleResponse,
} from "./util";

export async function getReviews(
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
