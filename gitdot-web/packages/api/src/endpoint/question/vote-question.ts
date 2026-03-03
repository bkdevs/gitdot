import { z } from "zod";
import { VoteResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const VoteQuestionRequest = z.object({
  value: z.number().int(),
});
export type VoteQuestionRequest = z.infer<typeof VoteQuestionRequest>;

export const VoteQuestionResponse = VoteResource;
export type VoteQuestionResponse = z.infer<typeof VoteQuestionResponse>;

export const VoteQuestion = {
  path: "/repository/{owner}/{repo}/question/{number}/vote",
  method: "POST",
  request: VoteQuestionRequest,
  response: VoteQuestionResponse,
} as const satisfies Endpoint;
export type VoteQuestion = typeof VoteQuestion;
