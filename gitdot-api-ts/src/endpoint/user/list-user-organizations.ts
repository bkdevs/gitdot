import { z } from "zod";

import { OrganizationResource } from "../../resource";

export const ListUserOrganizationsRequest = z.object({});
export type ListUserOrganizationsRequest = z.infer<
  typeof ListUserOrganizationsRequest
>;

export const ListUserOrganizations = {
  path: "/user/{user_name}/organizations",
  method: "GET",
  request: ListUserOrganizationsRequest,
  response: z.array(OrganizationResource),
} as const;
export type ListUserOrganizations = typeof ListUserOrganizations;
