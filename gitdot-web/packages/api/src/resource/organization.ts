import { z } from "zod";

export const OrganizationResource = z.object({
  id: z.uuid(),
  name: z.string(),
  created_at: z.iso.datetime(),
});
export type OrganizationResource = z.infer<typeof OrganizationResource>;

export const OrganizationMemberResource = z.object({
  id: z.uuid(),
  user_id: z.uuid(),
  organization_id: z.uuid(),
  role: z.string(),
  created_at: z.iso.datetime(),
});
export type OrganizationMemberResource = z.infer<
  typeof OrganizationMemberResource
>;
