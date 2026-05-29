import { z } from "zod";
import { page, UserRepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserContributedRepositoriesRequest = z.object({
  from: z.iso.datetime().optional(),
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListUserContributedRepositoriesRequest = z.infer<
  typeof ListUserContributedRepositoriesRequest
>;

export const ListUserContributedRepositoriesResponse = page(
  UserRepositoryResource,
);
export type ListUserContributedRepositoriesResponse = z.infer<
  typeof ListUserContributedRepositoriesResponse
>;

export const ListUserContributedRepositories = {
  path: "/user/{user_name}/repositories-contributed",
  method: "GET",
  request: ListUserContributedRepositoriesRequest,
  response: ListUserContributedRepositoriesResponse,
} as const satisfies Endpoint;
export type ListUserContributedRepositories =
  typeof ListUserContributedRepositories;
