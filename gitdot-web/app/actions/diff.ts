"use server";

import type { RepositoryDiffFileResource } from "gitdot-api";
import type { Element } from "hast";
import {
  createChangeMaps,
  type DiffHunk,
  diffFiles,
  fileToHast,
  inferLanguage,
  renderSpans,
} from "@/(main)/[owner]/[repo]/util";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitDiff,
  getReviewDiff,
} from "@/dal";

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

export async function renderCommitDiffAction(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const totalStart = performance.now();
  const fetchStart = performance.now();
  const result = await getRepositoryCommitDiff(owner, repo, sha);
  console.log(
    `[renderCommitDiffAction] getRepositoryCommitDiff: ${(performance.now() - fetchStart).toFixed(1)}ms`,
  );
  if (!result) {
    console.log(
      `[renderCommitDiffAction] total (no result): ${(performance.now() - totalStart).toFixed(1)}ms`,
    );
    return [];
  }
  console.log(
    `[renderCommitDiffAction] files to render: ${result.files.length}`,
  );
  const renderStart = performance.now();
  const entries = await renderDiffs(result.files);
  console.log(
    `[renderCommitDiffAction] renderDiffs: ${(performance.now() - renderStart).toFixed(1)}ms`,
  );
  console.log(
    `[renderCommitDiffAction] total: ${(performance.now() - totalStart).toFixed(1)}ms`,
  );
  return entries;
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
  const fileStart = performance.now();
  const left = file.left_content ?? null;
  const right = file.right_content ?? null;

  const lang = inferLanguage(file.path);
  const diffStart = performance.now();
  const hunks = left != null && right != null ? diffFiles(left, right) : [];
  console.log(
    `[renderDiff] ${file.path} diffFiles: ${(performance.now() - diffStart).toFixed(1)}ms (left=${left?.length ?? 0}, right=${right?.length ?? 0}, hunks=${hunks.length})`,
  );

  if (left && right && hunks.length > 0) {
    // anchor pairs (both lhs and rhs present) are context, not changes;
    // a hunk is all-additions iff no pair is lhs-only, all-removals iff no pair is rhs-only
    const isAllAdditions = hunks.every((h) =>
      h.every((p) => !(p.lhs && !p.rhs)),
    );
    const isAllRemovals = hunks.every((h) =>
      h.every((p) => !(p.rhs && !p.lhs)),
    );

    if (isAllAdditions || isAllRemovals) {
      const side = isAllAdditions ? ("right" as const) : ("left" as const);
      const content = isAllAdditions ? right : left;
      const mapStart = performance.now();
      const { leftLines, rightLines } = createChangeMaps(hunks);
      console.log(
        `[renderDiff] ${file.path} createChangeMaps: ${(performance.now() - mapStart).toFixed(1)}ms`,
      );
      const changedLines = isAllAdditions ? rightLines : leftLines;
      const spansStart = performance.now();
      const spans = await renderSpans(side, content, lang, changedLines);
      console.log(
        `[renderDiff] ${file.path} renderSpans (unilateral): ${(performance.now() - spansStart).toFixed(1)}ms`,
      );
      console.log(
        `[renderDiff] ${file.path} total: ${(performance.now() - fileStart).toFixed(1)}ms`,
      );
      return {
        kind: "unilateral" as const,
        spans,
        hunks,
        side,
      };
    }

    const mapStart = performance.now();
    const { leftLines, rightLines } = createChangeMaps(hunks);
    console.log(
      `[renderDiff] ${file.path} createChangeMaps: ${(performance.now() - mapStart).toFixed(1)}ms`,
    );
    const spansStart = performance.now();
    const [leftSpans, rightSpans] = await Promise.all([
      renderSpans("left", left, lang, leftLines),
      renderSpans("right", right, lang, rightLines),
    ]);
    console.log(
      `[renderDiff] ${file.path} renderSpans (split, parallel): ${(performance.now() - spansStart).toFixed(1)}ms`,
    );
    console.log(
      `[renderDiff] ${file.path} total: ${(performance.now() - fileStart).toFixed(1)}ms`,
    );
    return {
      kind: "split" as const,
      leftSpans,
      rightSpans,
      hunks,
    };
  } else if (left != null || right != null) {
    // biome-ignore lint/style/noNonNullAssertion: guaranteed non-null by the `left != null || right != null` condition
    const content = (left ?? right)!;
    const side = left != null ? "left" : "right";
    const lineType = side === "left" ? "removed" : "added";
    const hastStart = performance.now();
    const hast = await fileToHast(content, lang, "vitesse", [
      {
        line(node, lineNumber) {
          node.type = "element";
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;
          node.properties["data-line-type"] = lineType;
        },
      },
    ]);
    console.log(
      `[renderDiff] ${file.path} fileToHast (created/deleted): ${(performance.now() - hastStart).toFixed(1)}ms`,
    );
    const pre = hast.children[0] as Element;
    const code = pre.children[0] as Element;
    const spans = code.children.filter(
      (child): child is Element => child.type === "element",
    );
    console.log(
      `[renderDiff] ${file.path} total: ${(performance.now() - fileStart).toFixed(1)}ms`,
    );
    if (side === "left") return { kind: "deleted" as const };
    return { kind: "created" as const, spans };
  } else {
    console.log(
      `[renderDiff] ${file.path} no-change total: ${(performance.now() - fileStart).toFixed(1)}ms`,
    );
    return { kind: "no-change" as const };
  }
}
