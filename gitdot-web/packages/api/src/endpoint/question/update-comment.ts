import { z } from "zod";
import { CommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateCommentRequest = z.object({
  body: z.string(),
});
export type UpdateCommentRequest = z.infer<typeof UpdateCommentRequest>;

export const UpdateCommentResponse = CommentResource;
export type UpdateCommentResponse = z.infer<typeof UpdateCommentResponse>;

export const UpdateComment = {
  path: "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}",
  method: "PATCH",
  request: UpdateCommentRequest,
  response: UpdateCommentResponse,
} as const satisfies Endpoint;
export type UpdateComment = typeof UpdateComment;
