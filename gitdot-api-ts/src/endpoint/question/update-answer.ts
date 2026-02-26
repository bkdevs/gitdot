import { z } from "zod";

import { AnswerResource } from "../../resource";

export const UpdateAnswerRequest = z.object({
  body: z.string(),
});
export type UpdateAnswerRequest = z.infer<typeof UpdateAnswerRequest>;

export const UpdateAnswer = {
  path: "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}",
  method: "PATCH",
  request: UpdateAnswerRequest,
  response: AnswerResource,
} as const;
export type UpdateAnswer = typeof UpdateAnswer;
