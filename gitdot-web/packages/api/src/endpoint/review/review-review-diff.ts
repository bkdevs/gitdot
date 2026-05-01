import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ReviewCommentInput = z.object({
  revision_id: z.string().uuid(),
  parent_id: z.string().uuid().optional(),
  body: z.string(),
  file_path: z.string().optional(),
  line_number_start: z.number().int().optional(),
  line_number_end: z.number().int().optional(),
  start_character: z.number().int().optional(),
  end_character: z.number().int().optional(),
  side: z.string().optional(),
});
export type ReviewCommentInput = z.infer<typeof ReviewCommentInput>;

export const ReviewReviewDiffRequest = z.object({
  action: z.enum(["comment", "approve", "reject"]),
  comments: z.array(ReviewCommentInput),
});
export type ReviewReviewDiffRequest = z.infer<typeof ReviewReviewDiffRequest>;

export const ReviewReviewDiffResponse = ReviewResource;
export type ReviewReviewDiffResponse = z.infer<typeof ReviewReviewDiffResponse>;

export const ReviewReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/review",
  method: "POST",
  request: ReviewReviewDiffRequest,
  response: ReviewReviewDiffResponse,
} as const satisfies Endpoint;
export type ReviewReviewDiff = typeof ReviewReviewDiff;
