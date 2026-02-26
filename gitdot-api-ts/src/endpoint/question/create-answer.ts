import { z } from "zod";

import { AnswerResource } from "../../resource";

export const CreateAnswerRequest = z.object({
  body: z.string(),
});
export type CreateAnswerRequest = z.infer<typeof CreateAnswerRequest>;

export const CreateAnswer = {
  path: "/repository/{owner}/{repo}/question/{number}/answer",
  method: "POST",
  request: CreateAnswerRequest,
  response: AnswerResource,
} as const;
export type CreateAnswer = typeof CreateAnswer;
