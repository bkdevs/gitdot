import { z } from "zod";
import { QuestionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateQuestionRequest = z.object({
  title: z.string(),
  body: z.string(),
});
export type CreateQuestionRequest = z.infer<typeof CreateQuestionRequest>;

export const CreateQuestionResponse = QuestionResource;
export type CreateQuestionResponse = z.infer<typeof CreateQuestionResponse>;

export const CreateQuestion = {
  path: "/repository/{owner}/{repo}/question",
  method: "POST",
  request: CreateQuestionRequest,
  response: CreateQuestionResponse,
} as const satisfies Endpoint;
export type CreateQuestion = typeof CreateQuestion;
