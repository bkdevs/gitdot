import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const MergeReviewRequest = z.object({});
export type MergeReviewRequest = z.infer<typeof MergeReviewRequest>;

export const MergeReviewResponse = ReviewResource;
export type MergeReviewResponse = z.infer<typeof MergeReviewResponse>;

export const MergeReview = {
  path: "/repository/{owner}/{repo}/review/{number}/merge",
  method: "POST",
  request: MergeReviewRequest,
  response: MergeReviewResponse,
} as const satisfies Endpoint;
export type MergeReview = typeof MergeReview;
