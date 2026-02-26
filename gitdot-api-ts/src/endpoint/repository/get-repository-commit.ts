import { z } from "zod";

import { RepositoryCommitResource } from "../../resource";

export const GetRepositoryCommitRequest = z.object({});
export type GetRepositoryCommitRequest = z.infer<
  typeof GetRepositoryCommitRequest
>;

export const GetRepositoryCommit = {
  path: "/repository/{owner}/{repo}/commits/{sha}",
  method: "GET",
  request: GetRepositoryCommitRequest,
  response: RepositoryCommitResource,
} as const;
export type GetRepositoryCommit = typeof GetRepositoryCommit;
