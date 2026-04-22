import { z } from "zod";
import { ReviewCommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ReviewCommentInput = z.object({
  diff_id: z.string().uuid(),
  revision_id: z.string().uuid(),
  body: z.string(),
  file_path: z.string().optional(),
  line_number_start: z.number().int().optional(),
  line_number_end: z.number().int().optional(),
  start_character: z.number().int().optional(),
  end_character: z.number().int().optional(),
  side: z.string().optional(),
});
export type ReviewCommentInput = z.infer<typeof ReviewCommentInput>;

export const PublishReviewCommentsRequest = z.object({
  comments: z.array(ReviewCommentInput),
});
export type PublishReviewCommentsRequest = z.infer<
  typeof PublishReviewCommentsRequest
>;

export const PublishReviewCommentsResponse = z.array(ReviewCommentResource);
export type PublishReviewCommentsResponse = z.infer<
  typeof PublishReviewCommentsResponse
>;

export const PublishReviewComments = {
  path: "/repository/{owner}/{repo}/review/{number}/comments",
  method: "POST",
  request: PublishReviewCommentsRequest,
  response: PublishReviewCommentsResponse,
} as const satisfies Endpoint;
export type PublishReviewComments = typeof PublishReviewComments;
