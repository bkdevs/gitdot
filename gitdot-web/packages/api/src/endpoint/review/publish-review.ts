import { z } from "zod";
import type { Endpoint } from "../endpoint";

export const PublishReviewRequest = z.object({});
export type PublishReviewRequest = z.infer<typeof PublishReviewRequest>;

export const PublishReviewResponse = z.void();
export type PublishReviewResponse = z.infer<typeof PublishReviewResponse>;

export const PublishReview = {
  path: "/repository/{owner}/{repo}/review/{number}/publish",
  method: "POST",
  request: PublishReviewRequest,
  response: PublishReviewResponse,
} as const satisfies Endpoint;
export type PublishReview = typeof PublishReview;
