import { z } from "zod";
import { RepositoryCommitDiffResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryCommitDiffRequest = z.object({});
export type GetRepositoryCommitDiffRequest = z.infer<
  typeof GetRepositoryCommitDiffRequest
>;

export const GetRepositoryCommitDiffResponse = z.array(
  RepositoryCommitDiffResource,
);
export type GetRepositoryCommitDiffResponse = z.infer<
  typeof GetRepositoryCommitDiffResponse
>;

export const GetRepositoryCommitDiff = {
  path: "/repository/{owner}/{repo}/commits/{sha}/diff",
  method: "GET",
  request: GetRepositoryCommitDiffRequest,
  response: GetRepositoryCommitDiffResponse,
} as const satisfies Endpoint;
export type GetRepositoryCommitDiff = typeof GetRepositoryCommitDiff;
