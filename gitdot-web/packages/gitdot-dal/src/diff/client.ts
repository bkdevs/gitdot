import type {
  RepositoryBlobPairResource,
  RepositoryBlobResource,
} from "gitdot-api";

export async function fetchCommitBlobs(
  owner: string,
  repo: string,
  sha: string,
): Promise<RepositoryBlobPairResource[]> {
  const params = new URLSearchParams({ owner, repo, sha });
  return fetch(`/api/repository/diff?${params}`).then((res) => res.json());
}

export async function fetchFileBlobs(
  owner: string,
  repo: string,
  path: string,
  refs: string[],
): Promise<RepositoryBlobResource[]> {
  const params = new URLSearchParams({ owner, repo, path });
  for (const ref of refs) params.append("ref", ref);
  return fetch(`/api/repository/blobs?${params}`).then((res) => res.json());
}
