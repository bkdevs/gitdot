import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateReviewDiffRequest = z.object({
  message: z.string().optional(),
});
export type UpdateReviewDiffRequest = z.infer<typeof UpdateReviewDiffRequest>;

export const UpdateReviewDiffResponse = ReviewResource;
export type UpdateReviewDiffResponse = z.infer<typeof UpdateReviewDiffResponse>;

export const UpdateReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}",
  method: "PATCH",
  request: UpdateReviewDiffRequest,
  response: UpdateReviewDiffResponse,
} as const satisfies Endpoint;
export type UpdateReviewDiff = typeof UpdateReviewDiff;
