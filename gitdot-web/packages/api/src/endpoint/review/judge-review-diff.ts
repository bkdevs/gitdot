import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const JudgeReviewDiffRequest = z.object({
  verdict: z.enum(["approve", "reject"]),
});
export type JudgeReviewDiffRequest = z.infer<typeof JudgeReviewDiffRequest>;

export const JudgeReviewDiffResponse = ReviewResource;
export type JudgeReviewDiffResponse = z.infer<typeof JudgeReviewDiffResponse>;

export const JudgeReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/submit",
  method: "POST",
  request: JudgeReviewDiffRequest,
  response: JudgeReviewDiffResponse,
} as const satisfies Endpoint;
export type JudgeReviewDiff = typeof JudgeReviewDiff;
