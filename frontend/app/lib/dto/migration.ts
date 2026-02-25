import { z } from "zod";

export const GitHubInstallationResponseSchema = z.object({
  id: z.uuid(),
  installation_id: z.number(),
  owner_id: z.uuid(),
  installation_type: z.string(),
  github_login: z.string(),
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

export const GitHubInstallationListResponseSchema = z.array(
  GitHubInstallationResponseSchema,
);

export type GitHubInstallationListResponse = z.infer<
  typeof GitHubInstallationListResponseSchema
>;

export const GitHubRepositoryListResponseSchema = z.array(
  GitHubRepositoryResponseSchema,
);

export type GitHubRepositoryListResponse = z.infer<
  typeof GitHubRepositoryListResponseSchema
>;

export const MigrationRepositoryResponseSchema = z.object({
  id: z.uuid(),
  repository_id: z.uuid().nullable(),
  full_name: z.string(),
  status: z.string(),
  error: z.string().nullable(),
  created_at: z.string(),
  updated_at: z.string(),
});

export const MigrationResponseSchema = z.object({
  id: z.uuid(),
  author_id: z.uuid(),
  origin: z.string(),
  status: z.string(),
  repositories: z.array(MigrationRepositoryResponseSchema),
  created_at: z.string(),
  updated_at: z.string(),
});

export type MigrationResponse = z.infer<typeof MigrationResponseSchema>;

export const MigrationListResponseSchema = z.array(MigrationResponseSchema);

export type MigrationListResponse = z.infer<typeof MigrationListResponseSchema>;
