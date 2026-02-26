import { z } from "zod";
import { RepositoryCommitsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryCommitsRequest = z.object({
  ref_name: z.string(),
  page: z.number().int(),
  per_page: z.number().int(),
});
export type GetRepositoryCommitsRequest = z.infer<
  typeof GetRepositoryCommitsRequest
>;

export const GetRepositoryCommitsResponse = RepositoryCommitsResource;
export type GetRepositoryCommitsResponse = z.infer<
  typeof GetRepositoryCommitsResponse
>;

export const GetRepositoryCommits = {
  path: "/repository/{owner}/{repo}/commits",
  method: "GET",
  request: GetRepositoryCommitsRequest,
  response: GetRepositoryCommitsResponse,
} satisfies Endpoint;
export type GetRepositoryCommits = typeof GetRepositoryCommits;
