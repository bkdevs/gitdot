import { z } from "zod";

import { RepositoryCommitStatResource } from "../../resource";

export const GetRepositoryCommitStatRequest = z.object({});
export type GetRepositoryCommitStatRequest = z.infer<
  typeof GetRepositoryCommitStatRequest
>;

export const GetRepositoryCommitStat = {
  path: "/repository/{owner}/{repo}/commits/{sha}/stat",
  method: "GET",
  request: GetRepositoryCommitStatRequest,
  response: z.array(RepositoryCommitStatResource),
} as const;
export type GetRepositoryCommitStat = typeof GetRepositoryCommitStat;
