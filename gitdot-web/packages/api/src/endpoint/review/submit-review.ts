import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const SubmitReviewComment = z.object({
  body: z.string(),
  parent_id: z.uuid().nullable().optional(),
  file_path: z.string().nullable().optional(),
  line_number_start: z.number().int().nullable().optional(),
  line_number_end: z.number().int().nullable().optional(),
  side: z.string().nullable().optional(),
});
export type SubmitReviewComment = z.infer<typeof SubmitReviewComment>;

export const SubmitReviewRequest = z.object({
  action: z.string(),
  comments: z.array(SubmitReviewComment),
});
export type SubmitReviewRequest = z.infer<typeof SubmitReviewRequest>;

export const SubmitReviewResponse = ReviewResource;
export type SubmitReviewResponse = z.infer<typeof SubmitReviewResponse>;

export const SubmitReview = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/submit",
  method: "POST",
  request: SubmitReviewRequest,
  response: SubmitReviewResponse,
} as const satisfies Endpoint;
export type SubmitReview = typeof SubmitReview;
