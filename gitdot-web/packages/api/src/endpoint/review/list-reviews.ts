import { z } from "zod";
import { ReviewResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListReviewsRequest = z.object({});
export type ListReviewsRequest = z.infer<typeof ListReviewsRequest>;

export const ListReviewsResponse = z.array(ReviewResource);
export type ListReviewsResponse = z.infer<typeof ListReviewsResponse>;

export const ListReviews = {
  path: "/repository/{owner}/{repo}/reviews",
  method: "GET",
  request: ListReviewsRequest,
  response: ListReviewsResponse,
} as const satisfies Endpoint;
export type ListReviews = typeof ListReviews;
