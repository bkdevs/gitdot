import { z } from "zod";

import { VoteResource } from "../../resource";

export const VoteAnswerRequest = z.object({
  value: z.number().int(),
});
export type VoteAnswerRequest = z.infer<typeof VoteAnswerRequest>;

export const VoteAnswer = {
  path: "/repository/{owner}/{repo}/question/{number}/answer/{answer_id}/vote",
  method: "POST",
  request: VoteAnswerRequest,
  response: VoteResource,
} as const;
export type VoteAnswer = typeof VoteAnswer;
