import { z } from "zod";
import { MigrationResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

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

export const MigrateGitHubRepositoriesResponse = MigrationResource;
export type MigrateGitHubRepositoriesResponse = z.infer<
  typeof MigrateGitHubRepositoriesResponse
>;

export const MigrateGitHubRepositories = {
  path: "/migration/github/{installation_id}/migrate",
  method: "POST",
  request: MigrateGitHubRepositoriesRequest,
  response: MigrateGitHubRepositoriesResponse,
} satisfies Endpoint;
export type MigrateGitHubRepositories = typeof MigrateGitHubRepositories;
