import { z } from "zod";

import { QuestionResource } from "../../resource";

export const CreateQuestionRequest = z.object({
  title: z.string(),
  body: z.string(),
});
export type CreateQuestionRequest = z.infer<typeof CreateQuestionRequest>;

export const CreateQuestion = {
  path: "/repository/{owner}/{repo}/question",
  method: "POST",
  request: CreateQuestionRequest,
  response: QuestionResource,
} as const;
export type CreateQuestion = typeof CreateQuestion;
