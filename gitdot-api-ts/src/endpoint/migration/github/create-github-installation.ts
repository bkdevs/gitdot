import { z } from "zod";

import { GitHubInstallationResource } from "../../../resource";

export const CreateGitHubInstallationRequest = z.object({});
export type CreateGitHubInstallationRequest = z.infer<
  typeof CreateGitHubInstallationRequest
>;

export const CreateGitHubInstallation = {
  path: "/migration/github/{installation_id}",
  method: "POST",
  request: CreateGitHubInstallationRequest,
  response: GitHubInstallationResource,
} as const;
export type CreateGitHubInstallation = typeof CreateGitHubInstallation;
