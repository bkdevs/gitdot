import "server-only";

import { toQueryString } from "@/util";
import {
  type RepositoryCommits,
  type RepositoryCommitsQuery,
  RepositoryCommitsSchema,
  type RepositoryFile,
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
