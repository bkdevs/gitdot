import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const MergeReviewDiffRequest = z.object({});
export type MergeReviewDiffRequest = z.infer<typeof MergeReviewDiffRequest>;

export const MergeReviewDiffResponse = ReviewResource;
export type MergeReviewDiffResponse = z.infer<typeof MergeReviewDiffResponse>;

export const MergeReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/merge",
  method: "POST",
  request: MergeReviewDiffRequest,
  response: MergeReviewDiffResponse,
} as const satisfies Endpoint;
export type MergeReviewDiff = typeof MergeReviewDiff;
