import { z } from "zod";

import { OrganizationMemberResource } from "../../resource";

export const AddMemberRequest = z.object({
  user_name: z.string(),
  role: z.string(),
});
export type AddMemberRequest = z.infer<typeof AddMemberRequest>;

export const AddMember = {
  path: "/organization/{org_name}/repositories",
  method: "POST",
  request: AddMemberRequest,
  response: OrganizationMemberResource,
} as const;
export type AddMember = typeof AddMember;
