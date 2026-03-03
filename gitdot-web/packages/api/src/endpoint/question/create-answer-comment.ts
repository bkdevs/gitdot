import { z } from "zod";
import { CommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateAnswerCommentRequest = z.object({
  body: z.string(),
});
export type CreateAnswerCommentRequest = z.infer<
  typeof CreateAnswerCommentRequest
>;

export const CreateAnswerCommentResponse = CommentResource;
export type CreateAnswerCommentResponse = z.infer<
  typeof CreateAnswerCommentResponse
>;

export const CreateAnswerComment = {
  path: "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/comment",
  method: "POST",
  request: CreateAnswerCommentRequest,
  response: CreateAnswerCommentResponse,
} as const satisfies Endpoint;
export type CreateAnswerComment = typeof CreateAnswerComment;
