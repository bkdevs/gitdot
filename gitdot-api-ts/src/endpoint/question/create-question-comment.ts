import { z } from "zod";

import { CommentResource } from "../../resource";

export const CreateQuestionCommentRequest = z.object({
  body: z.string(),
});
export type CreateQuestionCommentRequest = z.infer<
  typeof CreateQuestionCommentRequest
>;

export const CreateQuestionComment = {
  path: "/repository/{owner}/{repo}/question/{number}/comment",
  method: "POST",
  request: CreateQuestionCommentRequest,
  response: CommentResource,
} as const;
export type CreateQuestionComment = typeof CreateQuestionComment;
