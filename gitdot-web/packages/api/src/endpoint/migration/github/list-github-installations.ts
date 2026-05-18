import { z } from "zod";
import { GitHubInstallationResource, page } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const ListGitHubInstallationsRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListGitHubInstallationsRequest = z.infer<
  typeof ListGitHubInstallationsRequest
>;

export const ListGitHubInstallationsResponse = page(GitHubInstallationResource);
export type ListGitHubInstallationsResponse = z.infer<
  typeof ListGitHubInstallationsResponse
>;

export const ListGitHubInstallations = {
  path: "/migration/github/installations",
  method: "GET",
  request: ListGitHubInstallationsRequest,
  response: ListGitHubInstallationsResponse,
} as const satisfies Endpoint;
export type ListGitHubInstallations = typeof ListGitHubInstallations;
