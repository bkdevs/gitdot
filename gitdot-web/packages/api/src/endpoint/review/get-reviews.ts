import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetReviewsRequest = z.object({});
export type GetReviewsRequest = z.infer<typeof GetReviewsRequest>;

export const GetReviewsResponse = z.array(ReviewResource);
export type GetReviewsResponse = z.infer<typeof GetReviewsResponse>;

export const GetReviews = {
  path: "/repository/{owner}/{repo}/reviews",
  method: "GET",
  request: GetReviewsRequest,
  response: GetReviewsResponse,
} as const satisfies Endpoint;
export type GetReviews = typeof GetReviews;
