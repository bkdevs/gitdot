import { z } from "zod";
import { RepositoryCommitsResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryFileCommitsRequest = z.object({
  path: z.string(),
  ref_name: z.string(),
  page: z.number().int(),
  per_page: z.number().int(),
});
export type GetRepositoryFileCommitsRequest = z.infer<
  typeof GetRepositoryFileCommitsRequest
>;

export const GetRepositoryFileCommitsResponse = RepositoryCommitsResource;
export type GetRepositoryFileCommitsResponse = z.infer<
  typeof GetRepositoryFileCommitsResponse
>;

export const GetRepositoryFileCommits = {
  path: "/repository/{owner}/{repo}/file/commits",
  method: "GET",
  request: GetRepositoryFileCommitsRequest,
  response: GetRepositoryFileCommitsResponse,
} satisfies Endpoint;
export type GetRepositoryFileCommits = typeof GetRepositoryFileCommits;
