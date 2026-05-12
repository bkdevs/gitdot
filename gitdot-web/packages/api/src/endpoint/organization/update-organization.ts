import { z } from "zod";
import { OrganizationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const UpdateOrganizationRequest = z.object({
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).optional(),
  display_name: z.string().nullable().optional(),
});
export type UpdateOrganizationRequest = z.infer<
  typeof UpdateOrganizationRequest
>;

export const UpdateOrganizationResponse = OrganizationResource;
export type UpdateOrganizationResponse = z.infer<
  typeof UpdateOrganizationResponse
>;

export const UpdateOrganization = {
  path: "/organization/{org_name}",
  method: "PATCH",
  request: UpdateOrganizationRequest,
  response: UpdateOrganizationResponse,
} as const satisfies Endpoint;
export type UpdateOrganization = typeof UpdateOrganization;
