import { z } from "zod";

import { RepositoryCommitsResource } from "../../resource";

export const GetRepositoryCommitsRequest = z.object({
  ref_name: z.string(),
  page: z.number().int(),
  per_page: z.number().int(),
});
export type GetRepositoryCommitsRequest = z.infer<
  typeof GetRepositoryCommitsRequest
>;

export const GetRepositoryCommits = {
  path: "/repository/{owner}/{repo}/commits",
  method: "GET",
  request: GetRepositoryCommitsRequest,
  response: RepositoryCommitsResource,
} as const;
export type GetRepositoryCommits = typeof GetRepositoryCommits;
