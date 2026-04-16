import { z } from "zod";
import { ReviewerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const AddReviewReviewerRequest = z.object({
  user_name: z.string(),
});
export type AddReviewReviewerRequest = z.infer<typeof AddReviewReviewerRequest>;

export const AddReviewReviewerResponse = ReviewerResource;
export type AddReviewReviewerResponse = z.infer<
  typeof AddReviewReviewerResponse
>;

export const AddReviewReviewer = {
  path: "/repository/{owner}/{repo}/review/{number}/reviewer",
  method: "POST",
  request: AddReviewReviewerRequest,
  response: AddReviewReviewerResponse,
} as const satisfies Endpoint;
export type AddReviewReviewer = typeof AddReviewReviewer;
