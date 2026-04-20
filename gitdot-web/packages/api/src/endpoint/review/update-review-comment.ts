import { z } from "zod";
import { ReviewCommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateReviewCommentRequest = z.object({
  body: z.string(),
});
export type UpdateReviewCommentRequest = z.infer<
  typeof UpdateReviewCommentRequest
>;

export const UpdateReviewCommentResponse = ReviewCommentResource;
export type UpdateReviewCommentResponse = z.infer<
  typeof UpdateReviewCommentResponse
>;

export const UpdateReviewComment = {
  path: "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}",
  method: "PATCH",
  request: UpdateReviewCommentRequest,
  response: UpdateReviewCommentResponse,
} as const satisfies Endpoint;
export type UpdateReviewComment = typeof UpdateReviewComment;
