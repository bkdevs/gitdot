import { z } from "zod";
import { RepositoryCommitResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserCommitsRequest = z.object({});
export type ListUserCommitsRequest = z.infer<typeof ListUserCommitsRequest>;

export const ListUserCommitsResponse = z.array(RepositoryCommitResource);
export type ListUserCommitsResponse = z.infer<typeof ListUserCommitsResponse>;

export const ListUserCommits = {
  path: "/user/{user_name}/commits",
  method: "GET",
  request: ListUserCommitsRequest,
  response: ListUserCommitsResponse,
} as const satisfies Endpoint;
export type ListUserCommits = typeof ListUserCommits;
