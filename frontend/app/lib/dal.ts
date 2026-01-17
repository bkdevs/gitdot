import "server-only";

import { toQueryString } from "@/util";
import {
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
} from "./dto";
import { getSession } from "./supabase";

const API_BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";

export async function getRepositoryFile(
  owner: string,
  repo: string,
  query: RepositoryFileQuery,
): Promise<RepositoryFile | null> {
  if (!owner || !repo || !query.path) {
    console.error("Invalid getRepositoryFile request:", {
      owner,
      repo,
      query,
    });
    return null;
  }

  const session = await getSession();
  if (!session) return null;

  const queryString = toQueryString(query);
  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/file?${queryString}`,
  );

  if (!response.ok) {
    console.error(
      "wgetRepositoryFile failed:",
      response.status,
      response.statusText,
    );
    return null;
  }

  return RepositoryFileSchema.parse(await response.json());
}

export async function getRepositoryTree(
  owner: string,
  repo: string,
  query?: RepositoryTreeQuery,
): Promise<RepositoryTree | null> {
  if (!owner || !repo) {
    console.error("Invalid getRepositoryTree request: ", {
      owner,
      repo,
      query,
    });
    return null;
  }

  const session = await getSession();
  if (!session) return null;

  const queryString = toQueryString(query);
  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/tree?${queryString}`,
  );

  if (!response.ok) {
    console.error(
      "getRepositoryTree failed:",
      response.status,
      response.statusText,
    );
    return null;
  }

  return RepositoryTreeSchema.parse(await response.json());
}

export async function getRepositoryCommits(
  owner: string,
  repo: string,
  query?: RepositoryCommitsQuery,
): Promise<RepositoryCommits | null> {
  if (!owner || !repo) {
    console.error("Invalid getRepositoryCommits request: ", {
      owner,
      repo,
      query,
    });
    return null;
  }

  const session = await getSession();
  if (!session) return null;

  const queryString = toQueryString(query);
  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/commits?${queryString}`,
  );

  if (!response.ok) {
    console.error(
      "getRepositoryTree failed:",
      response.status,
      response.statusText,
    );
    return null;
  }

  return RepositoryCommitsSchema.parse(await response.json());
}

export async function getRepositoryFileCommits(
  owner: string,
  repo: string,
  query: RepositoryFileCommitsQuery,
): Promise<RepositoryCommits | null> {
  if (!owner || !repo || !query.path) {
    console.error("Invalid getRepositoryFileCommits request:", {
      owner,
      repo,
      query,
    });
    return null;
  }

  const session = await getSession();
  if (!session) return null;

  const queryString = toQueryString(query);
  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/file/commits?${queryString}`,
  );

  if (!response.ok) {
    console.error(
      "getRepositoryFileCommits failed:",
      response.status,
      response.statusText,
    );
    return null;
  }

  return RepositoryCommitsSchema.parse(await response.json());
}

export async function getRepositoryCommitStats(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffs | null> {
  if (!owner || !repo || !sha) {
    console.error("Invalid getRepositoryCommitStats request:", {
      owner,
      repo,
      sha,
    });
    return null;
  }

  const session = await getSession();
  if (!session) return null;

  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/commits/${sha}/stats`,
  );

  if (!response.ok) {
    console.error(
      "getRepositoryCommitStats failed:",
      response.status,
      response.statusText,
    );
    return null;
  }

  return RepositoryCommitDiffsSchema.parse(await response.json());
}

export async function getRepositoryCommitDiffs(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryCommitDiffs | null> {
  if (!owner || !repo || !sha) {
    console.error("Invalid getRepositoryCommitDiffs request:", {
      owner,
      repo,
      sha,
    });
    return null;
  }

  const session = await getSession();
  if (!session) return null;

  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/commits/${sha}/diffs`,
  );

  if (!response.ok) {
    console.error(
      "getRepositoryCommitDiffs failed:",
      response.status,
      response.statusText,
    );
    return null;
  }

  return RepositoryCommitDiffsSchema.parse(await response.json());
}
