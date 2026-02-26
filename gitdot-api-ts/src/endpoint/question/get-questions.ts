import { z } from "zod";

import { QuestionResource } from "../../resource";

export const GetQuestionsRequest = z.object({});
export type GetQuestionsRequest = z.infer<typeof GetQuestionsRequest>;

export const GetQuestions = {
  path: "/repository/{owner}/{repo}/questions",
  method: "GET",
  request: GetQuestionsRequest,
  response: z.array(QuestionResource),
} as const;
export type GetQuestions = typeof GetQuestions;
