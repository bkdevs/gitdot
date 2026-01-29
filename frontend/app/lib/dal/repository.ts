import "server-only";

import { toQueryString } from "@/util";
import {
  type CreateRepositoryRequest,
  type CreateRepositoryResponse,
  CreateRepositoryResponseSchema,
  type RepositoryCommitDiffs,
  RepositoryCommitDiffsSchema,
  type RepositoryCommits,
  type RepositoryCommitsQuery,
  RepositoryCommitsSchema,
  type RepositoryFile,
  type RepositoryFileCommitsQuery,
  type RepositoryFileQuery,
  RepositoryFileSchema,
  type RepositoryTree,
  type RepositoryTreeQuery,
  RepositoryTreeSchema,
} from "../dto";
import { GITDOT_SERVER_URL, authFetch, authPost, handleResponse } from "./util";

export async function createRepository(
  owner: string,
  repo: string,
  request: CreateRepositoryRequest,
): Promise<CreateRepositoryResponse | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}`,
    request,
  );

  return await handleResponse(response, CreateRepositoryResponseSchema);
}

export async function getRepositoryFile(
  owner: string,
  repo: string,
  query: RepositoryFileQuery,
): Promise<RepositoryFile | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/file?${queryString}`,
  );

  return await handleResponse(response, RepositoryFileSchema);
}

export async function getRepositoryTree(
  owner: string,
  repo: string,
  query?: RepositoryTreeQuery,
): Promise<RepositoryTree | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/tree?${queryString}`,
  );

  return await handleResponse(response, RepositoryTreeSchema);
}

export async function getRepositoryCommits(
  owner: string,
  repo: string,
  query?: RepositoryCommitsQuery,
): Promise<RepositoryCommits | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits?${queryString}`,
  );

  return await handleResponse(response, RepositoryCommitsSchema);
}

export async function getRepositoryFileCommits(
  owner: string,
  repo: string,
  query: RepositoryFileCommitsQuery,
): Promise<RepositoryCommits | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/file/commits?${queryString}`,
  );

  return await handleResponse(response, RepositoryCommitsSchema);
}

export async function getRepositoryCommitStats(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffs | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/stats`,
  );

  return await handleResponse(response, RepositoryCommitDiffsSchema);
}

export async function getRepositoryCommitDiffs(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffs | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/diffs`,
  );

  return await handleResponse(response, RepositoryCommitDiffsSchema);
}
