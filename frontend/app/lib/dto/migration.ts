import { z } from "zod";

export const GitHubInstallationResponseSchema = z.object({
  id: z.uuid(),
  installation_id: z.number(),
  owner_id: z.uuid(),
  installation_type: z.string(),
  created_at: z.string(),
});

export type GitHubInstallationResponse = z.infer<
  typeof GitHubInstallationResponseSchema
>;
