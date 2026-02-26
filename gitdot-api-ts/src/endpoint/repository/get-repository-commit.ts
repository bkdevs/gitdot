import { z } from "zod";
import { RepositoryCommitResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const GetRepositoryCommitRequest = z.object({});
export type GetRepositoryCommitRequest = z.infer<
  typeof GetRepositoryCommitRequest
>;

export const GetRepositoryCommitResponse = RepositoryCommitResource;
export type GetRepositoryCommitResponse = z.infer<
  typeof GetRepositoryCommitResponse
>;

export const GetRepositoryCommit = {
  path: "/repository/{owner}/{repo}/commits/{sha}",
  method: "GET",
  request: GetRepositoryCommitRequest,
  response: GetRepositoryCommitResponse,
} satisfies Endpoint;
export type GetRepositoryCommit = typeof GetRepositoryCommit;
