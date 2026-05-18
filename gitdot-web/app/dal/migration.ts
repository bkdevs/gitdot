import "server-only";

import {
  GitHubInstallationResource,
  GitHubRepositoryResource,
  ListGitHubInstallationsResponse,
  ListMigrationsResponse,
  MigrationResource,
} from "gitdot-api";
import { z } from "zod";
import { toQueryString } from "@/util";
import { authFetch, authPost, GITDOT_SERVER_URL, handleResponse } from "./util";

export async function listInstallations(opts?: {
  cursor?: string;
  limit?: number;
}): Promise<ListGitHubInstallationsResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/migration/github/installations${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListGitHubInstallationsResponse);
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

export async function listMigrations(opts?: {
  cursor?: string;
  limit?: number;
}): Promise<ListMigrationsResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/migrations${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListMigrationsResponse);
}

export async function migrateGitHubRepositories(
  installationId: number,
  origin: string,
  originType: string,
  destination: string,
  destinationType: string,
  repositories: { name: string; id: number }[],
  readonly: boolean,
): Promise<MigrationResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/migration/github/${installationId}/migrate`,
    {
      origin,
      origin_type: originType,
      destination,
      destination_type: destinationType,
      repositories,
      readonly,
    },
  );

  return await handleResponse(response, MigrationResource);
}
