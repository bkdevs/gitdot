import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateReviewRequest = z.object({
  title: z.string().optional(),
  description: z.string().optional(),
});
export type UpdateReviewRequest = z.infer<typeof UpdateReviewRequest>;

export const UpdateReviewResponse = ReviewResource;
export type UpdateReviewResponse = z.infer<typeof UpdateReviewResponse>;

export const UpdateReview = {
  path: "/repository/{owner}/{repo}/review/{number}",
  method: "PATCH",
  request: UpdateReviewRequest,
  response: UpdateReviewResponse,
} as const satisfies Endpoint;
export type UpdateReview = typeof UpdateReview;
