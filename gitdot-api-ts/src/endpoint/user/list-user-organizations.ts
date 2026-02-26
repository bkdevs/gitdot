import { z } from "zod";
import { OrganizationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListUserOrganizationsRequest = z.object({});
export type ListUserOrganizationsRequest = z.infer<
  typeof ListUserOrganizationsRequest
>;

export const ListUserOrganizationsResponse = z.array(OrganizationResource);
export type ListUserOrganizationsResponse = z.infer<
  typeof ListUserOrganizationsResponse
>;

export const ListUserOrganizations = {
  path: "/user/{user_name}/organizations",
  method: "GET",
  request: ListUserOrganizationsRequest,
  response: ListUserOrganizationsResponse,
} satisfies Endpoint;
export type ListUserOrganizations = typeof ListUserOrganizations;
