import "server-only";

import {
  type GitHubInstallationResponse,
  GitHubInstallationResponseSchema,
  type GitHubRepositoryListResponse,
  GitHubRepositoryListResponseSchema,
} from "../dto/migration";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function createInstallation(
  installationId: number,
): Promise<GitHubInstallationResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}`,
    {},
  );

  return await handleResponse(response, GitHubInstallationResponseSchema);
}

export async function listInstallationRepositories(
  installationId: number,
): Promise<GitHubRepositoryListResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}/repositories`,
  );

  return await handleResponse(response, GitHubRepositoryListResponseSchema);
}
