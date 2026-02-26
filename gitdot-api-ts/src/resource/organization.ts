import { z } from "zod";

export const OrganizationResource = z.object({
  id: z.string().uuid(),
  name: z.string(),
  created_at: z.string().datetime(),
});
export type OrganizationResource = z.infer<typeof OrganizationResource>;

export const OrganizationMemberResource = z.object({
  id: z.string().uuid(),
  user_id: z.string().uuid(),
  organization_id: z.string().uuid(),
  role: z.string(),
  created_at: z.string().datetime(),
});
export type OrganizationMemberResource = z.infer<
  typeof OrganizationMemberResource
>;
