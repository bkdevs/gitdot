import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";
import { ReviewCommentInput } from "./create-review-comments";

export { ReviewCommentInput };

export const ReviewReviewDiffRequest = z.object({
  action: z.enum(["comment", "approve", "reject"]),
  comments: z.array(ReviewCommentInput),
});
export type ReviewReviewDiffRequest = z.infer<typeof ReviewReviewDiffRequest>;

export const ReviewReviewDiffResponse = ReviewResource;
export type ReviewReviewDiffResponse = z.infer<typeof ReviewReviewDiffResponse>;

export const ReviewReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/review",
  method: "POST",
  request: ReviewReviewDiffRequest,
  response: ReviewReviewDiffResponse,
} as const satisfies Endpoint;
export type ReviewReviewDiff = typeof ReviewReviewDiff;
