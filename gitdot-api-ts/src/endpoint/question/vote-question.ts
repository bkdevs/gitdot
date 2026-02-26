import { z } from "zod";

import { VoteResource } from "../../resource";

export const VoteQuestionRequest = z.object({
  value: z.number().int(),
});
export type VoteQuestionRequest = z.infer<typeof VoteQuestionRequest>;

export const VoteQuestion = {
  path: "/repository/{owner}/{repo}/question/{number}/vote",
  method: "POST",
  request: VoteQuestionRequest,
  response: VoteResource,
} as const;
export type VoteQuestion = typeof VoteQuestion;
