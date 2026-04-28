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

export const CreateReviewCommentsRequest = z.object({
  comments: z.array(ReviewCommentInput),
});
export type CreateReviewCommentsRequest = z.infer<
  typeof CreateReviewCommentsRequest
>;

export const CreateReviewCommentsResponse = ReviewResource;
export type CreateReviewCommentsResponse = z.infer<
  typeof CreateReviewCommentsResponse
>;

export const CreateReviewComments = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}/comments",
  method: "POST",
  request: CreateReviewCommentsRequest,
  response: CreateReviewCommentsResponse,
} as const satisfies Endpoint;
export type CreateReviewComments = typeof CreateReviewComments;
