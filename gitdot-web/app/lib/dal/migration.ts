import "server-only";

import {
  GitHubInstallationResource,
  GitHubRepositoryResource,
  MigrationResource,
} from "gitdot-api-ts";
import { z } from "zod";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function listInstallations(): Promise<
  GitHubInstallationResource[] | null
> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/migration/github/installations`,
  );

  return await handleResponse(response, z.array(GitHubInstallationResource));
}

export async function createInstallation(
  installationId: number,
): Promise<GitHubInstallationResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}`,
    {},
  );

  return await handleResponse(response, GitHubInstallationResource);
}

export async function listInstallationRepositories(
  installationId: number,
): Promise<GitHubRepositoryResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}/repositories`,
  );

  return await handleResponse(response, z.array(GitHubRepositoryResource));
}

export async function getMigration(
  number: number,
): Promise<MigrationResource | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/migration/${number}`);

  return await handleResponse(response, MigrationResource);
}

export async function listMigrations(): Promise<MigrationResource[] | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/migrations`);

  return await handleResponse(response, z.array(MigrationResource));
}

export async function migrateGitHubRepositories(
  installationId: number,
  origin: string,
  originType: string,
  destination: string,
  destinationType: string,
  repositories: string[],
): Promise<MigrationResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}/migrate`,
    {
      origin,
      origin_type: originType,
      destination,
      destination_type: destinationType,
      repositories,
    },
  );

  return await handleResponse(response, MigrationResource);
}
