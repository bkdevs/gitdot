import { z } from "zod";
import { RepositoryCommitFilterResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListRepositoryCommitFiltersRequest = z.object({});
export type ListRepositoryCommitFiltersRequest = z.infer<
  typeof ListRepositoryCommitFiltersRequest
>;

export const ListRepositoryCommitFiltersResponse = z.array(
  RepositoryCommitFilterResource,
);
export type ListRepositoryCommitFiltersResponse = z.infer<
  typeof ListRepositoryCommitFiltersResponse
>;

export const ListRepositoryCommitFilters = {
  path: "/repository/{owner}/{repo}/commit_filters",
  method: "GET",
  request: ListRepositoryCommitFiltersRequest,
  response: ListRepositoryCommitFiltersResponse,
} as const satisfies Endpoint;
export type ListRepositoryCommitFilters = typeof ListRepositoryCommitFilters;
