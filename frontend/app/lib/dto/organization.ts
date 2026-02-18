import { z } from "zod";

export const OrganizationResponseSchema = z.object({
  id: z.uuid(),
  name: z.string(),
  created_at: z.iso.datetime(),
});

export type OrganizationResponse = z.infer<typeof OrganizationResponseSchema>;

export const UserOrganizationsResponseSchema = z.array(
  OrganizationResponseSchema,
);

export type UserOrganizationsResponse = z.infer<
  typeof UserOrganizationsResponseSchema
>;
