import { z } from "zod";

import { MigrationResource } from "../../../resource";

export const MigrateGitHubRepositoriesRequest = z.object({
  origin: z.string(),
  origin_type: z.string(),
  destination: z.string(),
  destination_type: z.string(),
  repositories: z.array(z.string()),
});
export type MigrateGitHubRepositoriesRequest = z.infer<
  typeof MigrateGitHubRepositoriesRequest
>;

export const MigrateGitHubRepositories = {
  path: "/migration/github/{installation_id}/migrate",
  method: "POST",
  request: MigrateGitHubRepositoriesRequest,
  response: MigrationResource,
} as const;
export type MigrateGitHubRepositories = typeof MigrateGitHubRepositories;
