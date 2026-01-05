import "server-only";

import { RepositoryFile, RepositoryFileQuery, RepositoryFileQuerySchema, RepositoryFileSchema } from "./dto";
import { getSession } from "./supabase";

const API_BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";

export async function getRepositoryFile(owner: string, repo: string, query: RepositoryFileQuery): Promise<RepositoryFile | null> {
  if (!owner || !repo || !query.path) {
    console.error("Invalid getFile request: ", { owner, repo, query });
    return null;
  }

  const session = await getSession();
  if (!session) { // for now, reject unauthenticated requests
    return null;
  }

  const queryParams = new URLSearchParams(query).toString();
  const response = await fetch(`${API_BASE_URL}/repository/${owner}/${repo}/file?${queryParams}`);

  if (!response.ok) {
    console.error("getFile request failed: ", response.status, response.statusText);
    return null;
  }

  return RepositoryFileSchema.parse(await response.json());
}
