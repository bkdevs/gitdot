import "server-only";

import {
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

  const queryParams = new URLSearchParams(query).toString();
  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/file?${queryParams}`,
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

  const queryParams = new URLSearchParams(query).toString();
  const response = await fetch(
    `${API_BASE_URL}/repository/${owner}/${repo}/tree?${queryParams}`,
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
