import { z } from "zod";

export const OrganizationResource = z.object({
  id: z.uuid(),
  name: z.string(),
  created_at: z.iso.datetime(),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  display_name: z.string().nullable().optional(),
});
export type OrganizationResource = z.infer<typeof OrganizationResource>;

export const OrganizationMemberResource = z.object({
  id: z.uuid(),
  user_id: z.uuid(),
  organization_id: z.uuid(),
  org_name: z.string(),
  role: z.string(),
  role_description: z.string().nullable().optional(),
  created_at: z.iso.datetime(),
  user_name: z.string(),
});
export type OrganizationMemberResource = z.infer<
  typeof OrganizationMemberResource
>;
