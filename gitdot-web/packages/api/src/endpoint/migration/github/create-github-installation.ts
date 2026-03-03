import { z } from "zod";
import { GitHubInstallationResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const CreateGitHubInstallationRequest = z.object({});
export type CreateGitHubInstallationRequest = z.infer<
  typeof CreateGitHubInstallationRequest
>;

export const CreateGitHubInstallationResponse = GitHubInstallationResource;
export type CreateGitHubInstallationResponse = z.infer<
  typeof CreateGitHubInstallationResponse
>;

export const CreateGitHubInstallation = {
  path: "/migration/github/{installation_id}",
  method: "POST",
  request: CreateGitHubInstallationRequest,
  response: CreateGitHubInstallationResponse,
} as const satisfies Endpoint;
export type CreateGitHubInstallation = typeof CreateGitHubInstallation;
