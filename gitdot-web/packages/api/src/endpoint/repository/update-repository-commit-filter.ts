import { z } from "zod";
import { RepositoryCommitFilterResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateRepositoryCommitFilterRequest = z.object({
  name: z.string(),
  authors: z.array(z.string()).optional(),
  tags: z.array(z.string()).optional(),
  paths: z.array(z.string()).optional(),
});
export type UpdateRepositoryCommitFilterRequest = z.infer<
  typeof UpdateRepositoryCommitFilterRequest
>;

export const UpdateRepositoryCommitFilterResponse =
  RepositoryCommitFilterResource;
export type UpdateRepositoryCommitFilterResponse = z.infer<
  typeof UpdateRepositoryCommitFilterResponse
>;

export const UpdateRepositoryCommitFilter = {
  path: "/repository/{owner}/{repo}/commit_filters/{filter_id}",
  method: "PATCH",
  request: UpdateRepositoryCommitFilterRequest,
  response: UpdateRepositoryCommitFilterResponse,
} as const satisfies Endpoint;
export type UpdateRepositoryCommitFilter = typeof UpdateRepositoryCommitFilter;
