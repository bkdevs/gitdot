import { z } from "zod";

import { CommentResource } from "../../resource";

export const CreateAnswerCommentRequest = z.object({
  body: z.string(),
});
export type CreateAnswerCommentRequest = z.infer<
  typeof CreateAnswerCommentRequest
>;

export const CreateAnswerComment = {
  path: "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/comment",
  method: "POST",
  request: CreateAnswerCommentRequest,
  response: CommentResource,
} as const;
export type CreateAnswerComment = typeof CreateAnswerComment;
