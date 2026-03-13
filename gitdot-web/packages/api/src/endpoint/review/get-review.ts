import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetReviewRequest = z.object({});
export type GetReviewRequest = z.infer<typeof GetReviewRequest>;

export const GetReviewResponse = ReviewResource;
export type GetReviewResponse = z.infer<typeof GetReviewResponse>;

export const GetReview = {
  path: "/repository/{owner}/{repo}/review/{number}",
  method: "GET",
  request: GetReviewRequest,
  response: GetReviewResponse,
} as const satisfies Endpoint;
export type GetReview = typeof GetReview;
