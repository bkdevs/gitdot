import { z } from "zod";

import { RepositoryResource } from "../../resource";

export const ListUserRepositoriesRequest = z.object({});
export type ListUserRepositoriesRequest = z.infer<
  typeof ListUserRepositoriesRequest
>;

export const ListUserRepositories = {
  path: "/user/{user_name}/repositories",
  method: "GET",
  request: ListUserRepositoriesRequest,
  response: z.array(RepositoryResource),
} as const;
export type ListUserRepositories = typeof ListUserRepositories;
