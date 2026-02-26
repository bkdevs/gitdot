import { z } from "zod";
import { AnswerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateAnswerRequest = z.object({
  body: z.string(),
});
export type CreateAnswerRequest = z.infer<typeof CreateAnswerRequest>;

export const CreateAnswerResponse = AnswerResource;
export type CreateAnswerResponse = z.infer<typeof CreateAnswerResponse>;

export const CreateAnswer = {
  path: "/repository/{owner}/{repo}/question/{number}/answer",
  method: "POST",
  request: CreateAnswerRequest,
  response: CreateAnswerResponse,
} satisfies Endpoint;
export type CreateAnswer = typeof CreateAnswer;
