import { z } from "zod";

import { OrganizationResource } from "../../resource";

export const CreateOrganizationRequest = z.object({});
export type CreateOrganizationRequest = z.infer<
  typeof CreateOrganizationRequest
>;

export const CreateOrganization = {
  path: "/organization/{org_name}",
  method: "POST",
  request: CreateOrganizationRequest,
  response: OrganizationResource,
} as const;
export type CreateOrganization = typeof CreateOrganization;
