import { z } from "zod";

export const OrganizationMemberResource = z.object({
  id: z.uuid(),
  user_id: z.uuid(),
  user_name: z.string(),

  role: z.string(),
  role_description: z.string().nullable().optional(),

  created_at: z.iso.datetime(),
  image_updated_at: z.iso.datetime(),
});
export type OrganizationMemberResource = z.infer<
  typeof OrganizationMemberResource
>;

export const OrganizationResource = z.object({
  id: z.uuid(),
  name: z.string(),

  display_name: z.string().nullable().optional(),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),

  created_at: z.iso.datetime(),
  image_updated_at: z.iso.datetime(),

  members: z.array(OrganizationMemberResource).nullable().optional(),
});
export type OrganizationResource = z.infer<typeof OrganizationResource>;
