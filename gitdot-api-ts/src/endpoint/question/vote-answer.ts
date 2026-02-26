import { z } from "zod";
import { VoteResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const VoteAnswerRequest = z.object({
  value: z.number().int(),
});
export type VoteAnswerRequest = z.infer<typeof VoteAnswerRequest>;

export const VoteAnswerResponse = VoteResource;
export type VoteAnswerResponse = z.infer<typeof VoteAnswerResponse>;

export const VoteAnswer = {
  path: "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/vote",
  method: "POST",
  request: VoteAnswerRequest,
  response: VoteAnswerResponse,
} satisfies Endpoint;
export type VoteAnswer = typeof VoteAnswer;
