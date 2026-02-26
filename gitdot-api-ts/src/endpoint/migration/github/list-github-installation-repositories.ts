import { z } from "zod";
import { GitHubRepositoryResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const ListGitHubInstallationRepositoriesRequest = z.object({});
export type ListGitHubInstallationRepositoriesRequest = z.infer<
  typeof ListGitHubInstallationRepositoriesRequest
>;

export const ListGitHubInstallationRepositoriesResponse = z.array(
  GitHubRepositoryResource,
);
export type ListGitHubInstallationRepositoriesResponse = z.infer<
  typeof ListGitHubInstallationRepositoriesResponse
>;

export const ListGitHubInstallationRepositories = {
  path: "/migration/github/{installation_id}/repositories",
  method: "GET",
  request: ListGitHubInstallationRepositoriesRequest,
  response: ListGitHubInstallationRepositoriesResponse,
} satisfies Endpoint;
export type ListGitHubInstallationRepositories =
  typeof ListGitHubInstallationRepositories;
