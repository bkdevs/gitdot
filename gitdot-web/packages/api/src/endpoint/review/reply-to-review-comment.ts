import { z } from "zod";
import { ReviewCommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ReplyToReviewCommentRequest = z.object({
  body: z.string(),
});
export type ReplyToReviewCommentRequest = z.infer<
  typeof ReplyToReviewCommentRequest
>;

export const ReplyToReviewCommentResponse = ReviewCommentResource;
export type ReplyToReviewCommentResponse = z.infer<
  typeof ReplyToReviewCommentResponse
>;

export const ReplyToReviewComment = {
  path: "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}/reply",
  method: "POST",
  request: ReplyToReviewCommentRequest,
  response: ReplyToReviewCommentResponse,
} as const satisfies Endpoint;
export type ReplyToReviewComment = typeof ReplyToReviewComment;
