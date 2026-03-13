import "server-only";

import { ReviewResource } from "gitdot-api";
import { z } from "zod";
import { authFetch, GITDOT_SERVER_URL, handleResponse } from "./util";

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
