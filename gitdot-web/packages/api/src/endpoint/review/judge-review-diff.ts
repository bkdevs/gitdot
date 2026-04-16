import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const JudgeReviewDiffComment = z.object({
  body: z.string(),
  parent_id: z.uuid().nullable().optional(),
  file_path: z.string().nullable().optional(),
  line_number_start: z.number().int().nullable().optional(),
  line_number_end: z.number().int().nullable().optional(),
  side: z.string().nullable().optional(),
});
export type JudgeReviewDiffComment = z.infer<typeof JudgeReviewDiffComment>;

export const JudgeReviewDiffRequest = z.object({
  action: z.string(),
  comments: z.array(JudgeReviewDiffComment),
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
