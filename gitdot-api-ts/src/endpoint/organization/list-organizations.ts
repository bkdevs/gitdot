import { z } from "zod";

import { OrganizationResource } from "../../resource";

export const ListOrganizationsRequest = z.object({});
export type ListOrganizationsRequest = z.infer<typeof ListOrganizationsRequest>;

export const ListOrganizations = {
  path: "/organizations",
  method: "GET",
  request: ListOrganizationsRequest,
  response: z.array(OrganizationResource),
} as const;
export type ListOrganizations = typeof ListOrganizations;
