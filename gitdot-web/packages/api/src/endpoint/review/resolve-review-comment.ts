import { z } from "zod";
import { ReviewCommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ResolveReviewCommentRequest = z.object({
  resolved: z.boolean(),
});
export type ResolveReviewCommentRequest = z.infer<
  typeof ResolveReviewCommentRequest
>;

export const ResolveReviewCommentResponse = ReviewCommentResource;
export type ResolveReviewCommentResponse = z.infer<
  typeof ResolveReviewCommentResponse
>;

export const ResolveReviewComment = {
  path: "/repository/{owner}/{repo}/review/{number}/comment/{comment_id}/resolve",
  method: "POST",
  request: ResolveReviewCommentRequest,
  response: ResolveReviewCommentResponse,
} as const satisfies Endpoint;
export type ResolveReviewComment = typeof ResolveReviewComment;
