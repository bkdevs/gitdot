import "server-only";

import { getRepositoryCommitBlobs, getReviewDiffBlobs } from "gitdot-client";
import { renderDiff } from "./shiki";
import type { DiffData } from "./types";

export async function renderCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffData> {
  const pairs = await getRepositoryCommitBlobs(owner, repo, sha);
  if (!pairs) return [];
  return renderDiff(pairs);
}

export async function renderReviewDiff(
  owner: string,
  repo: string,
  number: number | string,
  position: number,
  revision?: number,
  compareTo?: number,
): Promise<DiffData> {
  const result = await getReviewDiffBlobs(
    owner,
    repo,
    number,
    position,
    revision,
    compareTo,
  );
  if (!result) return [];
  return renderDiff(result);
}
