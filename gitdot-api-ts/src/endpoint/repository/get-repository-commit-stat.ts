import { z } from "zod";
import { RepositoryCommitStatResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryCommitStatRequest = z.object({});
export type GetRepositoryCommitStatRequest = z.infer<
  typeof GetRepositoryCommitStatRequest
>;

export const GetRepositoryCommitStatResponse = z.array(
  RepositoryCommitStatResource,
);
export type GetRepositoryCommitStatResponse = z.infer<
  typeof GetRepositoryCommitStatResponse
>;

export const GetRepositoryCommitStat = {
  path: "/repository/{owner}/{repo}/commits/{sha}/stat",
  method: "GET",
  request: GetRepositoryCommitStatRequest,
  response: GetRepositoryCommitStatResponse,
} as const satisfies Endpoint;
export type GetRepositoryCommitStat = typeof GetRepositoryCommitStat;
