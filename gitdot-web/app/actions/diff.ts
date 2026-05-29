"use server";

import type { RepositoryDiffFileResource } from "gitdot-api";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitDiff,
  getReviewDiff,
} from "gitdot-client";
import { inferLanguage } from "gitdot-dal/client";
import type { Element } from "hast";
import { unstable_cache } from "next/cache";
import {
  type DiffHunk,
  diffFiles,
  renderSpans,
} from "@/(main)/[owner]/[repo]/util";

export type DiffSpans =
  | {
      kind: "split";
      leftSpans: Element[];
      rightSpans: Element[];
      hunks: DiffHunk[];
    }
  | {
      kind: "unilateral";
      spans: Element[];
      hunks: DiffHunk[];
      side: "left" | "right";
    }
  | { kind: "created"; spans: Element[] }
  | { kind: "deleted" }
  | { kind: "no-change" };

export type DiffEntry = {
  resource: RepositoryDiffFileResource;
  spans: DiffSpans;
};

export async function renderBlobDiffsAction(
  owner: string,
  repo: string,
  commitShas: string[],
  path: string,
): Promise<Record<string, DiffEntry>> {
  const result = await getRepositoryBlobDiffs(owner, repo, {
    commit_shas: commitShas,
    path,
  });
  if (!result) return {};
  const entries = await Promise.all(
    Object.entries(result.diffs).map(
      async ([sha, file]) =>
        [sha, { resource: file, spans: await renderDiff(file) }] as const,
    ),
  );
  return Object.fromEntries(entries);
}

// Next.js' data cache rejects any entry over 2MB (the rendered diff is
// measured via JSON.stringify). In dev this rejection throws and surfaces as a
// "failed to pipe response" error; in prod it silently skips the write. Keep
// headroom below 2MB for the cache-entry wrapper so the write never fails.
const MAX_CACHED_DIFF_BYTES = 1.5 * 1024 * 1024;

export async function renderCommitDiffAction(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const result = await getRepositoryCommitDiff(owner, repo, sha);
  if (!result) return [];

  // note: this is after the backend API call so authz works
  // diffs never change given a set of files, so this is consistent per.
  // Only cache when the rendered diff fits under the data cache limit;
  // oversized commits fall back to an uncached render so the response never
  // fails to pipe.
  const cached = await unstable_cache(
    async () => {
      const entries = await renderDiffs(result.files);
      return JSON.stringify(entries).length <= MAX_CACHED_DIFF_BYTES
        ? entries
        : null;
    },
    ["commit-diff", owner, repo, sha],
    {
      tags: [`commit-diff:${owner}/${repo}/${sha}`],
    },
  )();

  return cached ?? renderDiffs(result.files);
}

export async function renderReviewDiffAction(
  owner: string,
  repo: string,
  number: number | string,
  position: number,
  revision?: number,
  compareTo?: number,
): Promise<DiffEntry[]> {
  const result = await getReviewDiff(
    owner,
    repo,
    number,
    position,
    revision,
    compareTo,
  );
  if (!result) return [];
  return renderDiffs(result.files);
}

async function renderDiffs(
  files: RepositoryDiffFileResource[],
): Promise<DiffEntry[]> {
  const datas = await Promise.all(files.map(renderDiff));
  return files.map((file, i) => ({ resource: file, spans: datas[i] }));
}

async function renderDiff(
  file: RepositoryDiffFileResource,
): Promise<DiffSpans> {
  const left = file.left_content ?? null;
  const right = file.right_content ?? null;
  const lang = inferLanguage(file.path);

  if (left != null && right != null) {
    const hunks = diffFiles(left, right);
    if (hunks.length === 0) return { kind: "no-change" };

    const allRemovedLines = new Set(hunks.flatMap((h) => [...h.removedLines]));
    const allAddedLines = new Set(hunks.flatMap((h) => [...h.addedLines]));
    const isAllAdditions = allRemovedLines.size === 0;
    const isAllRemovals = allAddedLines.size === 0;

    if (isAllAdditions || isAllRemovals) {
      const side = isAllAdditions ? "right" : "left";
      const content = isAllAdditions ? right : left;
      const changedLines = isAllAdditions ? allAddedLines : allRemovedLines;
      const spans = await renderSpans(
        side,
        content,
        lang,
        changedLines,
        "vitesse",
      );
      return { kind: "unilateral", spans, hunks, side };
    }

    const [leftSpans, rightSpans] = await Promise.all([
      renderSpans("left", left, lang, allRemovedLines, "vitesse"),
      renderSpans("right", right, lang, allAddedLines, "vitesse"),
    ]);
    return { kind: "split", leftSpans, rightSpans, hunks };
  } else if (right === null) {
    return { kind: "deleted" };
  } else if (left === null) {
    const lineCount = right.split("\n").length;
    const allLines = new Set(Array.from({ length: lineCount }, (_, i) => i));
    const spans = await renderSpans("right", right, lang, allLines, "vitesse");
    return { kind: "created", spans };
  } else {
    return { kind: "no-change" };
  }
}
