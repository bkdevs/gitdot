import "server-only";

import {
  ConvertReadonlyRepositoryResponse,
  type CreateRepositoryCommitFilterRequest,
  CreateRepositoryCommitFilterResponse,
  type CreateRepositoryRequest,
  GetRepositoryActivityResponse,
  type GetRepositoryBlobDiffsRequest,
  type GetRepositoryBlobRequest,
  type GetRepositoryPathsRequest,
  type GetRepositoryResourcesRequest,
  ListLatestRepositoriesResponse,
  ListRepositoryCommitFiltersResponse,
  type ListRepositoryCommitsRequest,
  ListRepositoryCommitsResponse,
  ListTrendingRepositoriesResponse,
  RepositoryBlobDiffsResource,
  RepositoryBlobResource,
  RepositoryCommitDiffResource,
  type RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositoryResource,
  RepositoryResourcesResource,
  type UpdateRepositoryCommitFilterRequest,
  UpdateRepositoryCommitFilterResponse,
} from "gitdot-api";
import { toQueryString } from "@/util";
import {
  authDelete,
  authFetch,
  authPatch,
  authPost,
  GITDOT_SERVER_URL,
  handleEmptyResponse,
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

// TODO: support pagination for repositories that need more than 10K
export async function listRepositoryCommits(
  owner: string,
  repo: string,
  query?: ListRepositoryCommitsRequest,
): Promise<ListRepositoryCommitsResponse | null> {
  const queryString = toQueryString(query);
  const url = `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits${queryString ? `?${queryString}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListRepositoryCommitsResponse);
}

export async function listLatestRepositories(): Promise<ListLatestRepositoriesResponse | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/repository/latest`);
  return await handleResponse(response, ListLatestRepositoriesResponse);
}

export async function listTrendingRepositories(): Promise<ListTrendingRepositoriesResponse | null> {
  const response = await authFetch(`${GITDOT_SERVER_URL}/repository/trending`);
  return await handleResponse(response, ListTrendingRepositoriesResponse);
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

export async function listRepositoryCommitFilters(
  owner: string,
  repo: string,
  opts?: { cursor?: string; limit?: number },
): Promise<ListRepositoryCommitFiltersResponse | null> {
  const qs = toQueryString({ cursor: opts?.cursor, limit: opts?.limit });
  const url = `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commit-filters${qs ? `?${qs}` : ""}`;
  const response = await authFetch(url);
  return await handleResponse(response, ListRepositoryCommitFiltersResponse);
}

export async function createRepositoryCommitFilter(
  owner: string,
  repo: string,
  request: CreateRepositoryCommitFilterRequest,
): Promise<RepositoryCommitFilterResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commit-filters`,
    request,
  );
  return await handleResponse(response, CreateRepositoryCommitFilterResponse);
}

export async function updateRepositoryCommitFilter(
  owner: string,
  repo: string,
  filterId: string,
  request: UpdateRepositoryCommitFilterRequest,
): Promise<RepositoryCommitFilterResource | null> {
  const response = await authPatch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commit-filters/${filterId}`,
    request,
  );
  return await handleResponse(response, UpdateRepositoryCommitFilterResponse);
}

export async function deleteRepositoryCommitFilter(
  owner: string,
  repo: string,
  filterId: string,
): Promise<void> {
  const response = await authDelete(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commit-filters/${filterId}`,
  );
  await handleEmptyResponse(response);
}

export async function getRepositoryBlobDiffs(
  owner: string,
  repo: string,
  request: GetRepositoryBlobDiffsRequest,
): Promise<RepositoryBlobDiffsResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/blob/diffs`,
    request,
  );
  return await handleResponse(response, RepositoryBlobDiffsResource);
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

export async function getRepository(
  owner: string,
  repo: string,
): Promise<RepositoryResource | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}`,
  );
  return await handleResponse(response, RepositoryResource);
}

export async function getRepositoryActivity(
  owner: string,
  repo: string,
): Promise<GetRepositoryActivityResponse | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/activity`,
  );
  return await handleResponse(response, GetRepositoryActivityResponse);
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

export async function starRepository(
  owner: string,
  repo: string,
): Promise<void> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/star`,
    {},
  );
  await handleEmptyResponse(response);
}

export async function unstarRepository(
  owner: string,
  repo: string,
): Promise<void> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/unstar`,
    {},
  );
  await handleEmptyResponse(response);
}

export async function convertReadonlyRepository(
  owner: string,
  repo: string,
): Promise<ConvertReadonlyRepositoryResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/convert-readonly`,
    {},
  );
  return await handleResponse(response, ConvertReadonlyRepositoryResponse);
}
