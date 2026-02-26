import { z } from "zod";

import { GitHubInstallationResource } from "../../../resource";

export const ListGitHubInstallationsRequest = z.object({});
export type ListGitHubInstallationsRequest = z.infer<
  typeof ListGitHubInstallationsRequest
>;

export const ListGitHubInstallations = {
  path: "/migration/github/installations",
  method: "GET",
  request: ListGitHubInstallationsRequest,
  response: z.array(GitHubInstallationResource),
} as const;
export type ListGitHubInstallations = typeof ListGitHubInstallations;
