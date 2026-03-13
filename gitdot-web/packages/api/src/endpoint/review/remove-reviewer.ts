import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const RemoveReviewerRequest = z.object({});
export type RemoveReviewerRequest = z.infer<typeof RemoveReviewerRequest>;

export const RemoveReviewerResponse = z.void();
export type RemoveReviewerResponse = z.infer<typeof RemoveReviewerResponse>;

export const RemoveReviewer = {
  path: "/repository/{owner}/{repo}/review/{number}/reviewer/{reviewer_name}",
  method: "DELETE",
  request: RemoveReviewerRequest,
  response: RemoveReviewerResponse,
} as const satisfies Endpoint;
export type RemoveReviewer = typeof RemoveReviewer;
