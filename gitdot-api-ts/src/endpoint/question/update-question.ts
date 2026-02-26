import { z } from "zod";

import { QuestionResource } from "../../resource";

export const UpdateQuestionRequest = z.object({
  title: z.string(),
  body: z.string(),
});
export type UpdateQuestionRequest = z.infer<typeof UpdateQuestionRequest>;

export const UpdateQuestion = {
  path: "/repository/{owner}/{repo}/question/{number}",
  method: "PATCH",
  request: UpdateQuestionRequest,
  response: QuestionResource,
} as const;
export type UpdateQuestion = typeof UpdateQuestion;
