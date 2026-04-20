import { z } from "zod";
import { ReviewCommentResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateReviewCommentRequest = z.object({
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
export type CreateReviewCommentRequest = z.infer<
  typeof CreateReviewCommentRequest
>;

export const CreateReviewCommentResponse = ReviewCommentResource;
export type CreateReviewCommentResponse = z.infer<
  typeof CreateReviewCommentResponse
>;

export const CreateReviewComment = {
  path: "/repository/{owner}/{repo}/review/{number}/comment",
  method: "POST",
  request: CreateReviewCommentRequest,
  response: CreateReviewCommentResponse,
} as const satisfies Endpoint;
export type CreateReviewComment = typeof CreateReviewComment;
