import { z } from "zod";

import { GitHubRepositoryResource } from "../../../resource";

export const ListGitHubInstallationRepositoriesRequest = z.object({});
export type ListGitHubInstallationRepositoriesRequest = z.infer<
  typeof ListGitHubInstallationRepositoriesRequest
>;

export const ListGitHubInstallationRepositories = {
  path: "/migration/github/{installation_id}/repositories",
  method: "GET",
  request: ListGitHubInstallationRepositoriesRequest,
  response: z.array(GitHubRepositoryResource),
} as const;
export type ListGitHubInstallationRepositories =
  typeof ListGitHubInstallationRepositories;
