import { z } from "zod";
import { OrganizationMemberResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const AddMemberRequest = z.object({
  user_name: z.string(),
  role: z.string(),
});
export type AddMemberRequest = z.infer<typeof AddMemberRequest>;

export const AddMemberResponse = OrganizationMemberResource;
export type AddMemberResponse = z.infer<typeof AddMemberResponse>;

export const AddMember = {
  path: "/organization/{org_name}/repositories",
  method: "POST",
  request: AddMemberRequest,
  response: AddMemberResponse,
} as const satisfies Endpoint;
export type AddMember = typeof AddMember;
