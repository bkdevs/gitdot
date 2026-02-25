import "server-only";

import { z } from "zod";
import { toQueryString } from "@/util";
import {
  type CreateRepositoryRequest,
  type CreateRepositoryResponse,
  CreateRepositoryResponseSchema,
  type RepositoryCommit,
  type RepositoryCommitDiff,
  RepositoryCommitDiffSchema,
  RepositoryCommitSchema,
  type RepositoryCommitStat,
  RepositoryCommitStatSchema,
  type RepositoryCommits,
  type RepositoryCommitsQuery,
  RepositoryCommitsSchema,
  type RepositoryFile,
  type RepositoryFileCommitsQuery,
  type RepositoryFileQuery,
  RepositoryFileSchema,
  RepositoryPermissionResponseSchema,
  type RepositoryPreview,
  type RepositoryPreviewQuery,
  RepositoryPreviewSchema,
  type RepositoryTree,
  type RepositoryTreeQuery,
  RepositoryTreeSchema,
} from "../dto";
import {
  authDelete,
  authFetch,
  authPost,
  GITDOT_SERVER_URL,
  handleResponse,
  NotFound,
} from "./util";

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
): Promise<RepositoryFile | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/file?${queryString}`,
  );
  if (response.status === 404) return NotFound;

  return await handleResponse(response, RepositoryFileSchema);
}

export async function getRepositoryTree(
  owner: string,
  repo: string,
  query?: RepositoryTreeQuery,
): Promise<RepositoryTree | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/tree?${queryString}`,
  );
  if (response.status === 404) return NotFound;

  return await handleResponse(response, RepositoryTreeSchema);
}

export async function getRepositoryCommits(
  owner: string,
  repo: string,
  query?: RepositoryCommitsQuery,
): Promise<RepositoryCommits | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits?${queryString}`,
  );
  if (response.status === 404) return NotFound;

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

export async function getRepositoryCommit(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommit | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}`,
  );

  return await handleResponse(response, RepositoryCommitSchema);
}

export async function getRepositoryCommitStat(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitStat[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/stat`,
  );

  return await handleResponse(response, z.array(RepositoryCommitStatSchema));
}

export async function getRepositoryCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiff[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/diff`,
  );

  return await handleResponse(response, z.array(RepositoryCommitDiffSchema));
}

export async function getRepositoryPreview(
  owner: string,
  repo: string,
  query?: RepositoryPreviewQuery,
): Promise<RepositoryPreview | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/preview?${queryString}`,
  );
  if (response.status === 404) return NotFound;

  return await handleResponse(response, RepositoryPreviewSchema);
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

export async function getRepositoryPermission(
  owner: string,
  repo: string,
): Promise<string | null> {
  try {
    const response = await authFetch(
      `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/permission`,
    );
    if (!response.ok) return null;

    const data = await handleResponse(
      response,
      RepositoryPermissionResponseSchema,
    );
    return data?.permission ?? null;
  } catch {
    return null;
  }
}

export async function isRepositoryAdmin(
  owner: string,
  repo: string,
): Promise<boolean> {
  const permission = await getRepositoryPermission(owner, repo);
  return permission === "admin";
}
