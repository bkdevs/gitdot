import { z } from "zod";

export const GitHubInstallationResource = z.object({
  id: z.uuid(),
  installation_id: z.number(),
  owner_id: z.uuid(),
  installation_type: z.string(),
  github_login: z.string(),
  created_at: z.iso.datetime(),
});
export type GitHubInstallationResource = z.infer<
  typeof GitHubInstallationResource
>;

export const GitHubRepositoryResource = z.object({
  id: z.number(),
  name: z.string(),
  full_name: z.string(),
  description: z.string().nullable(),
  private: z.boolean(),
  default_branch: z.string(),
});
export type GitHubRepositoryResource = z.infer<typeof GitHubRepositoryResource>;

export const MigrationRepositoryResource = z.object({
  id: z.uuid(),
  origin_full_name: z.string(),
  destination_full_name: z.string(),
  visibility: z.string(),
  status: z.string(),
  error: z.string().nullable(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
});
export type MigrationRepositoryResource = z.infer<
  typeof MigrationRepositoryResource
>;

export const MigrationResource = z.object({
  id: z.uuid(),
  number: z.number().int(),
  author_id: z.uuid(),
  origin_service: z.string(),
  origin: z.string(),
  origin_type: z.string(),
  destination: z.string(),
  destination_type: z.string(),
  status: z.string(),
  created_at: z.iso.datetime(),
  updated_at: z.iso.datetime(),
  repositories: z.array(MigrationRepositoryResource),
});
export type MigrationResource = z.infer<typeof MigrationResource>;
