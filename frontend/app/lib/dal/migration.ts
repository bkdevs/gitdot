import "server-only";

import {
  type GitHubInstallationListResponse,
  GitHubInstallationListResponseSchema,
  type GitHubInstallationResponse,
  GitHubInstallationResponseSchema,
  type GitHubRepositoryListResponse,
  GitHubRepositoryListResponseSchema,
  type MigrationListResponse,
  MigrationListResponseSchema,
  type MigrationResponse,
  MigrationResponseSchema,
} from "../dto/migration";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function listInstallations(): Promise<GitHubInstallationListResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/migration/github/installations`,
  );

  return await handleResponse(response, GitHubInstallationListResponseSchema);
}

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

export async function getMigration(
  number: number,
): Promise<MigrationResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/migration/${number}`,
  );

  return await handleResponse(response, MigrationResponseSchema);
}

export async function listMigrations(): Promise<MigrationListResponse | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/migrations`);

  return await handleResponse(response, MigrationListResponseSchema);
}

export async function migrateGitHubRepositories(
  installationId: number,
  owner: string,
  ownerType: string,
  repositories: string[],
): Promise<MigrationResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}/migrate`,
    { owner, owner_type: ownerType, repositories },
  );

  return await handleResponse(response, MigrationResponseSchema);
}
