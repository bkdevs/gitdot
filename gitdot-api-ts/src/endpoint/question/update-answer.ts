import { z } from "zod";
import { AnswerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateAnswerRequest = z.object({
  body: z.string(),
});
export type UpdateAnswerRequest = z.infer<typeof UpdateAnswerRequest>;

export const UpdateAnswerResponse = AnswerResource;
export type UpdateAnswerResponse = z.infer<typeof UpdateAnswerResponse>;

export const UpdateAnswer = {
  path: "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}",
  method: "PATCH",
  request: UpdateAnswerRequest,
  response: UpdateAnswerResponse,
} satisfies Endpoint;
export type UpdateAnswer = typeof UpdateAnswer;
