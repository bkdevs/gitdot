import { z } from "zod";
import { QuestionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateQuestionRequest = z.object({
  title: z.string(),
  body: z.string(),
});
export type UpdateQuestionRequest = z.infer<typeof UpdateQuestionRequest>;

export const UpdateQuestionResponse = QuestionResource;
export type UpdateQuestionResponse = z.infer<typeof UpdateQuestionResponse>;

export const UpdateQuestion = {
  path: "/repository/{owner}/{repo}/question/{number}",
  method: "PATCH",
  request: UpdateQuestionRequest,
  response: UpdateQuestionResponse,
} satisfies Endpoint;
export type UpdateQuestion = typeof UpdateQuestion;
