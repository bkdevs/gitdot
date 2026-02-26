import { z } from "zod";

import { RepositoryCommitDiffResource } from "../../resource";

export const GetRepositoryCommitDiffRequest = z.object({});
export type GetRepositoryCommitDiffRequest = z.infer<
  typeof GetRepositoryCommitDiffRequest
>;

export const GetRepositoryCommitDiff = {
  path: "/repository/{owner}/{repo}/commits/{sha}/diff",
  method: "GET",
  request: GetRepositoryCommitDiffRequest,
  response: z.array(RepositoryCommitDiffResource),
} as const;
export type GetRepositoryCommitDiff = typeof GetRepositoryCommitDiff;
