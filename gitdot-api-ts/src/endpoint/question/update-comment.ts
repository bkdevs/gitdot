import { z } from "zod";

import { CommentResource } from "../../resource";

export const UpdateCommentRequest = z.object({
  body: z.string(),
});
export type UpdateCommentRequest = z.infer<typeof UpdateCommentRequest>;

export const UpdateComment = {
  path: "/repository/{owner}/{repo}/question/{number}/comment/{comment_id}",
  method: "PATCH",
  request: UpdateCommentRequest,
  response: CommentResource,
} as const;
export type UpdateComment = typeof UpdateComment;
