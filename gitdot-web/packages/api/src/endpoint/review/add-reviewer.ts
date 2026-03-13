import { z } from "zod";
import { ReviewerResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const AddReviewerRequest = z.object({
  user_name: z.string(),
});
export type AddReviewerRequest = z.infer<typeof AddReviewerRequest>;

export const AddReviewerResponse = ReviewerResource;
export type AddReviewerResponse = z.infer<typeof AddReviewerResponse>;

export const AddReviewer = {
  path: "/repository/{owner}/{repo}/review/{number}/reviewer",
  method: "POST",
  request: AddReviewerRequest,
  response: AddReviewerResponse,
} as const satisfies Endpoint;
export type AddReviewer = typeof AddReviewer;
