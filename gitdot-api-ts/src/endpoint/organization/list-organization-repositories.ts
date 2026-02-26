import { z } from "zod";

import { RepositoryResource } from "../../resource";

export const ListOrganizationRepositoriesRequest = z.object({});
export type ListOrganizationRepositoriesRequest = z.infer<
  typeof ListOrganizationRepositoriesRequest
>;

export const ListOrganizationRepositories = {
  path: "/organization/{org_name}/repositories",
  method: "GET",
  request: ListOrganizationRepositoriesRequest,
  response: z.array(RepositoryResource),
} as const;
export type ListOrganizationRepositories = typeof ListOrganizationRepositories;
