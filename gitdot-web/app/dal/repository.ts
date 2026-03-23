import "server-only";

import {
  type CreateRepositoryRequest,
  type GetRepositoryBlobRequest,
  type GetRepositoryBlobsRequest,
  type GetRepositoryCommitsRequest,
  type GetRepositoryFileCommitsRequest,
  type GetRepositoryPathsRequest,
  type GetRepositoryResourcesRequest,
  RepositoryBlobResource,
  RepositoryBlobsResource,
  RepositoryCommitDiffResource,
  RepositoryCommitResource,
  RepositoryCommitsResource,
  RepositoryPathsResource,
  RepositoryResource,
  RepositoryResourcesResource,
  RepositorySettingsResource,
  type UpdateRepositorySettingsRequest,
} from "gitdot-api";
import { toQueryString } from "@/util";
import {
  authDelete,
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
} from "./util";

export async function createRepository(
  owner: string,
  repo: string,
  request: CreateRepositoryRequest,
): Promise<RepositoryResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}`,
    request,
  );

  return await handleResponse(response, RepositoryResource);
}

export async function getRepositoryBlob(
  owner: string,
  repo: string,
  query: GetRepositoryBlobRequest,
): Promise<RepositoryBlobResource | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/blob?${queryString}`,
  );

  return await handleResponse(response, RepositoryBlobResource);
}

export async function getRepositoryCommits(
  owner: string,
  repo: string,
  query?: GetRepositoryCommitsRequest,
): Promise<RepositoryCommitsResource | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits?${queryString}`,
  );

  return await handleResponse(response, RepositoryCommitsResource);
}

export async function getRepositoryFileCommits(
  owner: string,
  repo: string,
  query: GetRepositoryFileCommitsRequest,
): Promise<RepositoryCommitsResource | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/file/commits?${queryString}`,
  );

  return await handleResponse(response, RepositoryCommitsResource);
}

export async function getRepositoryCommit(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}`,
  );

  return await handleResponse(response, RepositoryCommitResource);
}

export async function getRepositoryPaths(
  owner: string,
  repo: string,
  query?: GetRepositoryPathsRequest,
): Promise<RepositoryPathsResource | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/paths?${queryString}`,
  );
  return await handleResponse(response, RepositoryPathsResource);
}

export async function getRepositoryBlobs(
  owner: string,
  repo: string,
  request: GetRepositoryBlobsRequest,
): Promise<RepositoryBlobsResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/blobs`,
    request,
  );
  return await handleResponse(response, RepositoryBlobsResource);
}

export async function getRepositoryCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/diff`,
  );
  return await handleResponse(response, RepositoryCommitDiffResource);
}

export async function getRepositorySettings(
  owner: string,
  repo: string,
): Promise<RepositorySettingsResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/settings`,
  );
  return await handleResponse(response, RepositorySettingsResource);
}

export async function getRepositoryResources(
  owner: string,
  repo: string,
  request: GetRepositoryResourcesRequest = {},
): Promise<RepositoryResourcesResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/resources`,
    request,
  );
  return await handleResponse(response, RepositoryResourcesResource);
}

export async function updateRepositorySettings(
  owner: string,
  repo: string,
  request: UpdateRepositorySettingsRequest,
): Promise<RepositorySettingsResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/settings`,
    request,
  );

  return await handleResponse(response, RepositorySettingsResource);
}

export async function deleteRepository(
  owner: string,
  repo: string,
): Promise<void> {
  const response = await authDelete(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}`,
  );
  if (!response.ok) {
    let message = response.statusText;
    try {
      const body = await response.json();
      if (typeof body?.message === "string") message = body.message;
    } catch {}
    throw new Error(message);
  }
}
