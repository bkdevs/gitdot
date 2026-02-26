import { z } from "zod";

import { QuestionResource } from "../../resource";

export const GetQuestionRequest = z.object({});
export type GetQuestionRequest = z.infer<typeof GetQuestionRequest>;

export const GetQuestion = {
  path: "/repository/{owner}/{repo}/question/{number}",
  method: "GET",
  request: GetQuestionRequest,
  response: QuestionResource,
} as const;
export type GetQuestion = typeof GetQuestion;
