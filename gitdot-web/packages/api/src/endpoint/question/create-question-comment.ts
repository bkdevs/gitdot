import { z } from "zod";
import { CommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateQuestionCommentRequest = z.object({
  body: z.string(),
});
export type CreateQuestionCommentRequest = z.infer<
  typeof CreateQuestionCommentRequest
>;

export const CreateQuestionCommentResponse = CommentResource;
export type CreateQuestionCommentResponse = z.infer<
  typeof CreateQuestionCommentResponse
>;

export const CreateQuestionComment = {
  path: "/repository/{owner}/{repo}/question/{number}/comment",
  method: "POST",
  request: CreateQuestionCommentRequest,
  response: CreateQuestionCommentResponse,
} as const satisfies Endpoint;
export type CreateQuestionComment = typeof CreateQuestionComment;
