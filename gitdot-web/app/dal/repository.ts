import "server-only";

import {
  type CreateRepositoryRequest,
  type GetRepositoryCommitsRequest,
  type GetRepositoryFileCommitsRequest,
  type GetRepositoryFileRequest,
  type GetRepositoryPreviewRequest,
  type GetRepositoryTreeRequest,
  RepositoryCommitDiffResource,
  RepositoryCommitResource,
  RepositoryCommitStatResource,
  RepositoryCommitsResource,
  RepositoryFileResource,
  RepositoryPermissionResource,
  RepositoryPreviewResource,
  RepositoryResource,
  RepositoryTreeResource,
} from "gitdot-api";
import { z } from "zod";
import { toQueryString } from "@/util";
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
): Promise<RepositoryResource | null> {
  const response = await authPost(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}`,
    request,
  );

  return await handleResponse(response, RepositoryResource);
}

export async function getRepositoryFile(
  owner: string,
  repo: string,
  query: GetRepositoryFileRequest,
): Promise<RepositoryFileResource | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/file?${queryString}`,
  );
  if (response.status === 404) return NotFound;

  return await handleResponse(response, RepositoryFileResource);
}

export async function getRepositoryTree(
  owner: string,
  repo: string,
  query?: GetRepositoryTreeRequest,
): Promise<RepositoryTreeResource | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/tree?${queryString}`,
  );
  if (response.status === 404) return NotFound;

  return await handleResponse(response, RepositoryTreeResource);
}

export async function getRepositoryCommits(
  owner: string,
  repo: string,
  query?: GetRepositoryCommitsRequest,
): Promise<RepositoryCommitsResource | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits?${queryString}`,
  );
  if (response.status === 404) return NotFound;

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

export async function getRepositoryCommitStat(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitStatResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/stat`,
  );

  return await handleResponse(response, z.array(RepositoryCommitStatResource));
}

export async function getRepositoryCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffResource[] | null> {
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/commits/${sha}/diff`,
  );

  return await handleResponse(response, z.array(RepositoryCommitDiffResource));
}

export async function getRepositoryPreview(
  owner: string,
  repo: string,
  query?: GetRepositoryPreviewRequest,
): Promise<RepositoryPreviewResource | NotFound | null> {
  const queryString = toQueryString(query);
  const response = await authFetch(
    `${GITDOT_SERVER_URL}/repository/${owner}/${repo}/preview?${queryString}`,
  );
  if (response.status === 404) return NotFound;

  return await handleResponse(response, RepositoryPreviewResource);
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

    const data = await handleResponse(response, RepositoryPermissionResource);
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
