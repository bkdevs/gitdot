import { z } from "zod";
import { OrganizationResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const CreateOrganizationRequest = z.object({});
export type CreateOrganizationRequest = z.infer<
  typeof CreateOrganizationRequest
>;

export const CreateOrganizationResponse = OrganizationResource;
export type CreateOrganizationResponse = z.infer<
  typeof CreateOrganizationResponse
>;

export const CreateOrganization = {
  path: "/organization/{org_name}",
  method: "POST",
  request: CreateOrganizationRequest,
  response: CreateOrganizationResponse,
} as const satisfies Endpoint;
export type CreateOrganization = typeof CreateOrganization;
