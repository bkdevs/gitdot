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

export const GitHubRepositoryResponseSchema = z.object({
  id: z.number(),
  name: z.string(),
  full_name: z.string(),
  description: z.string().nullable(),
  private: z.boolean(),
  default_branch: z.string(),
});

export type GitHubRepositoryResponse = z.infer<
  typeof GitHubRepositoryResponseSchema
>;

export const GitHubRepositoryListResponseSchema = z.array(
  GitHubRepositoryResponseSchema,
);

export type GitHubRepositoryListResponse = z.infer<
  typeof GitHubRepositoryListResponseSchema
>;
