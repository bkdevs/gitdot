import { z } from "zod";
import { RepositoryCommitFilterResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateRepositoryCommitFilterRequest = z.object({
  name: z.string(),
  authors: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  paths: z.array(z.string()).optional(),
});
export type CreateRepositoryCommitFilterRequest = z.infer<
  typeof CreateRepositoryCommitFilterRequest
>;

export const CreateRepositoryCommitFilterResponse =
  RepositoryCommitFilterResource;
export type CreateRepositoryCommitFilterResponse = z.infer<
  typeof CreateRepositoryCommitFilterResponse
>;

export const CreateRepositoryCommitFilter = {
  path: "/repository/{owner}/{repo}/commit_filters",
  method: "POST",
  request: CreateRepositoryCommitFilterRequest,
  response: CreateRepositoryCommitFilterResponse,
} as const satisfies Endpoint;
export type CreateRepositoryCommitFilter = typeof CreateRepositoryCommitFilter;
