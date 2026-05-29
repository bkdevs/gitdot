import { z } from "zod";
import { page, RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserStarredRepositoriesRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListUserStarredRepositoriesRequest = z.infer<
  typeof ListUserStarredRepositoriesRequest
>;

export const ListUserStarredRepositoriesResponse = page(RepositoryResource);
export type ListUserStarredRepositoriesResponse = z.infer<
  typeof ListUserStarredRepositoriesResponse
>;

export const ListUserStarredRepositories = {
  path: "/user/{user_name}/repositories-starred",
  method: "GET",
  request: ListUserStarredRepositoriesRequest,
  response: ListUserStarredRepositoriesResponse,
} as const satisfies Endpoint;
export type ListUserStarredRepositories = typeof ListUserStarredRepositories;
