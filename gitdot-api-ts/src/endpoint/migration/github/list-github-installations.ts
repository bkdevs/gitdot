import { z } from "zod";
import { GitHubInstallationResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const ListGitHubInstallationsRequest = z.object({});
export type ListGitHubInstallationsRequest = z.infer<
  typeof ListGitHubInstallationsRequest
>;

export const ListGitHubInstallationsResponse = z.array(
  GitHubInstallationResource,
);
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
