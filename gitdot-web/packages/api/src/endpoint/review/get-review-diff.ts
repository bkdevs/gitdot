import { z } from "zod";
import { RepositoryDiffResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetReviewDiffResponse = z.object({
  files: z.array(RepositoryDiffResource),
});
export type GetReviewDiffResponse = z.infer<typeof GetReviewDiffResponse>;

export const GetReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}",
  method: "GET",
  request: z.object({}),
  response: GetReviewDiffResponse,
} as const satisfies Endpoint;
export type GetReviewDiff = typeof GetReviewDiff;
