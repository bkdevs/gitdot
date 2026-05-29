import { z } from "zod";

export const UserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  display_name: z.string().nullable().optional(),
  created_at: z.iso.datetime(),
  image_updated_at: z.iso.datetime(),
});
export type UserResource = z.infer<typeof UserResource>;

export const UserEmailResource = z.object({
  email: z.string(),
  is_primary: z.boolean(),
  is_verified: z.boolean(),
  created_at: z.iso.datetime(),
});
export type UserEmailResource = z.infer<typeof UserEmailResource>;

export const UserOrganizationResource = z.object({
  id: z.uuid(),
  name: z.string(),
  display_name: z.string().nullable().optional(),

  role: z.string(),
  role_description: z.string().nullable().optional(),
  joined_at: z.iso.datetime(),
  image_updated_at: z.iso.datetime(),
});
export type UserOrganizationResource = z.infer<typeof UserOrganizationResource>;

export const CurrentUserResource = z.object({
  id: z.uuid(),
  name: z.string(),
  emails: z.array(UserEmailResource),
  memberships: z.array(UserOrganizationResource),
  location: z.string().nullable().optional(),
  readme: z.string().nullable().optional(),
  links: z.array(z.string()).default([]),
  display_name: z.string().nullable().optional(),
  created_at: z.iso.datetime(),
  image_updated_at: z.iso.datetime(),
});
export type CurrentUserResource = z.infer<typeof CurrentUserResource>;
