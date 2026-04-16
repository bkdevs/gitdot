import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const RemoveReviewReviewerRequest = z.object({});
export type RemoveReviewReviewerRequest = z.infer<
  typeof RemoveReviewReviewerRequest
>;

export const RemoveReviewReviewerResponse = z.void();
export type RemoveReviewReviewerResponse = z.infer<
  typeof RemoveReviewReviewerResponse
>;

export const RemoveReviewReviewer = {
  path: "/repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}",
  method: "DELETE",
  request: RemoveReviewReviewerRequest,
  response: RemoveReviewReviewerResponse,
} as const satisfies Endpoint;
export type RemoveReviewReviewer = typeof RemoveReviewReviewer;
