import { z } from "zod";
import { RepositoryBlobDiffsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryBlobDiffsRequest = z.object({
  commit_shas: z.array(z.string()),
  path: z.string(),
});
export type GetRepositoryBlobDiffsRequest = z.infer<
  typeof GetRepositoryBlobDiffsRequest
>;

export const GetRepositoryBlobDiffsResponse = RepositoryBlobDiffsResource;
export type GetRepositoryBlobDiffsResponse = z.infer<
  typeof GetRepositoryBlobDiffsResponse
>;

export const GetRepositoryBlobDiffs = {
  path: "/repository/{owner}/{repo}/blob/diffs",
  method: "POST",
  request: GetRepositoryBlobDiffsRequest,
  response: GetRepositoryBlobDiffsResponse,
} as const satisfies Endpoint;
export type GetRepositoryBlobDiffs = typeof GetRepositoryBlobDiffs;
