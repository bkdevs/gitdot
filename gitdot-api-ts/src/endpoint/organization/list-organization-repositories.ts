import { z } from "zod";
import { RepositoryResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListOrganizationRepositoriesRequest = z.object({});
export type ListOrganizationRepositoriesRequest = z.infer<
  typeof ListOrganizationRepositoriesRequest
>;

export const ListOrganizationRepositoriesResponse = z.array(RepositoryResource);
export type ListOrganizationRepositoriesResponse = z.infer<
  typeof ListOrganizationRepositoriesResponse
>;

export const ListOrganizationRepositories = {
  path: "/organization/{org_name}/repositories",
  method: "GET",
  request: ListOrganizationRepositoriesRequest,
  response: ListOrganizationRepositoriesResponse,
} satisfies Endpoint;
export type ListOrganizationRepositories = typeof ListOrganizationRepositories;
