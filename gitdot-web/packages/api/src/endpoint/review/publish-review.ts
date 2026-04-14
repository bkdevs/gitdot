import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const DiffUpdate = z.object({
  position: z.number().int(),
  message: z.string().optional(),
});
export type DiffUpdate = z.infer<typeof DiffUpdate>;

export const PublishReviewRequest = z.object({
  title: z.string().optional(),
  description: z.string().optional(),
  diffs: z.array(DiffUpdate).optional(),
});
export type PublishReviewRequest = z.infer<typeof PublishReviewRequest>;

export const PublishReviewResponse = ReviewResource;
export type PublishReviewResponse = z.infer<typeof PublishReviewResponse>;

export const PublishReview = {
  path: "/repository/{owner}/{repo}/review/{number}/publish",
  method: "POST",
  request: PublishReviewRequest,
  response: PublishReviewResponse,
} as const satisfies Endpoint;
export type PublishReview = typeof PublishReview;
