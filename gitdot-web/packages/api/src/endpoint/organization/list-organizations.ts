import { z } from "zod";
import { OrganizationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListOrganizationsRequest = z.object({});
export type ListOrganizationsRequest = z.infer<typeof ListOrganizationsRequest>;

export const ListOrganizationsResponse = z.array(OrganizationResource);
export type ListOrganizationsResponse = z.infer<
  typeof ListOrganizationsResponse
>;

export const ListOrganizations = {
  path: "/organizations",
  method: "GET",
  request: ListOrganizationsRequest,
  response: ListOrganizationsResponse,
} as const satisfies Endpoint;
export type ListOrganizations = typeof ListOrganizations;
