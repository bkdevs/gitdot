import { z } from "zod";
import { GitHubInstallationResource } from "../../../resource";
import type { Endpoint } from "../../endpoint";

export const CreateGitHubInstallationRequest = z.object({
  state: z.string(),
  code: z.string(),
});
export type CreateGitHubInstallationRequest = z.infer<
  typeof CreateGitHubInstallationRequest
>;

export const CreateGitHubInstallationResponse = z.object({
  installation: GitHubInstallationResource,
  action: z.enum(["migration", "onboarding"]),
});
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
