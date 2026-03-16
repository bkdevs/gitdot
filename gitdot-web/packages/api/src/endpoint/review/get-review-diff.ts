import { z } from "zod";
import { RepositoryDiffFileResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetReviewDiffResponse = z.object({
  files: z.array(RepositoryDiffFileResource),
});
export type GetReviewDiffResponse = z.infer<typeof GetReviewDiffResponse>;

export const GetReviewDiff = {
  path: "/repository/{owner}/{repo}/review/{number}/diff/{position}",
  method: "GET",
  request: z.object({}),
  response: GetReviewDiffResponse,
} as const satisfies Endpoint;
export type GetReviewDiff = typeof GetReviewDiff;
