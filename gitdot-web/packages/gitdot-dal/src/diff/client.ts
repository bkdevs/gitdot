import type { DiffEntry } from "./types";

export function fetchCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const params = new URLSearchParams({ owner, repo, sha });
  return fetch(`/api/repository/diff?${params}`).then((res) => res.json());
}

export function fetchBlobDiffs(
  owner: string,
  repo: string,
  commitShas: string[],
  path: string,
): Promise<Record<string, DiffEntry>> {
  const params = new URLSearchParams({ owner, repo, path });
  for (const sha of commitShas) params.append("sha", sha);
  return fetch(`/api/repository/blob-diff?${params}`).then((res) => res.json());
}
