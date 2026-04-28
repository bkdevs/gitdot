import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";
import { ReviewCommentInput } from "./review-review-diff";

export const CreateReviewCommentsRequest = z.object({
  comments: z.array(ReviewCommentInput),
});
export type CreateReviewCommentsRequest = z.infer<
  typeof CreateReviewCommentsRequest
>;

export const CreateReviewCommentsResponse = ReviewResource;
export type CreateReviewCommentsResponse = z.infer<
  typeof CreateReviewCommentsResponse
>;

export const CreateReviewComments = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/comments",
  method: "POST",
  request: CreateReviewCommentsRequest,
  response: CreateReviewCommentsResponse,
} as const satisfies Endpoint;
export type CreateReviewComments = typeof CreateReviewComments;
