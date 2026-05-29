import "server-only";

import type { RepositoryDiffFileResource } from "gitdot-api";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitDiff,
  getReviewDiff,
} from "gitdot-client";
import { diffFiles } from "./algo";
import { inferLanguage, renderSpans } from "./shiki";
import type { DiffEntry, DiffSpans } from "./types";

export async function renderBlobDiffs(
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

export async function renderCommitDiff(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const result = await getRepositoryCommitDiff(owner, repo, sha);
  if (!result) return [];
  return renderDiffs(result.files);
}

export async function renderReviewDiff(
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
        content,
        lang,
        "vitesse",
        side,
        changedLines,
      );
      return { kind: "unilateral", spans, hunks, side };
    }

    const [leftSpans, rightSpans] = await Promise.all([
      renderSpans(left, lang, "vitesse", "left", allRemovedLines),
      renderSpans(right, lang, "vitesse", "right", allAddedLines),
    ]);
    return { kind: "split", leftSpans, rightSpans, hunks };
  } else if (right === null) {
    return { kind: "deleted" };
  } else if (left === null) {
    const lineCount = right.split("\n").length;
    const allLines = new Set(Array.from({ length: lineCount }, (_, i) => i));
    const spans = await renderSpans(right, lang, "vitesse", "right", allLines);
    return { kind: "created", spans };
  } else {
    return { kind: "no-change" };
  }
}
