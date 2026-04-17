"use server";

import type { DiffHunkResource, RepositoryDiffFileResource } from "gitdot-api";
import type { Element } from "hast";
import {
  createChangeMaps,
  fileToHast,
  inferLanguage,
  mergeHunks,
  renderSpans,
} from "@/(main)/[owner]/[repo]/util";
import {
  getRepositoryBlobDiffs,
  getRepositoryCommitDiff,
  getReviewDiff,
} from "@/dal";

export type DiffData =
  | {
      kind: "split";
      leftSpans: Element[];
      rightSpans: Element[];
      hunks: DiffHunkResource[];
    }
  | {
      kind: "unilateral";
      spans: Element[];
      hunks: DiffHunkResource[];
      side: "left" | "right";
    }
  | { kind: "created"; spans: Element[] }
  | { kind: "deleted" }
  | { kind: "no-change" };

export type DiffEntry = {
  diff: RepositoryDiffFileResource;
  data: DiffData;
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
        [sha, { diff: file, data: await renderDiff(file) }] as const,
    ),
  );
  return Object.fromEntries(entries);
}

export async function renderCommitDiffAction(
  owner: string,
  repo: string,
  sha: string,
): Promise<DiffEntry[]> {
  const result = await getRepositoryCommitDiff(owner, repo, sha);
  if (!result) return [];
  return renderDiffs(result.files);
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
  return files.map((file, i) => ({ diff: file, data: datas[i] }));
}

async function renderDiff(file: RepositoryDiffFileResource): Promise<DiffData> {
  const left = file.left_content ?? null;
  const right = file.right_content ?? null;

  const lang = inferLanguage(file.path);
  const processedHunks = mergeHunks(file.hunks);

  if (left && right && file.hunks.length > 0) {
    const isAllAdditions = processedHunks.every((h) => h.every((p) => !p.lhs));
    const isAllRemovals = processedHunks.every((h) => h.every((p) => !p.rhs));

    if (isAllAdditions || isAllRemovals) {
      const side = isAllAdditions ? ("right" as const) : ("left" as const);
      const content = isAllAdditions ? right : left;
      const { leftChangeMap, rightChangeMap } =
        createChangeMaps(processedHunks);
      const changeMap = isAllAdditions ? rightChangeMap : leftChangeMap;
      const spans = await renderSpans(side, content, lang, changeMap);
      return {
        kind: "unilateral" as const,
        spans,
        hunks: processedHunks,
        side,
      };
    }

    const { leftChangeMap, rightChangeMap } = createChangeMaps(processedHunks);
    const [leftSpans, rightSpans] = await Promise.all([
      renderSpans("left", left, lang, leftChangeMap),
      renderSpans("right", right, lang, rightChangeMap),
    ]);
    return {
      kind: "split" as const,
      leftSpans,
      rightSpans,
      hunks: processedHunks,
    };
  } else if (left != null || right != null) {
    // biome-ignore lint/style/noNonNullAssertion: guaranteed non-null by the `left != null || right != null` condition
    const content = (left ?? right)!;
    const side = left != null ? "left" : "right";
    const lineType = side === "left" ? "removed" : "added";
    const hast = await fileToHast(content, lang, "vitesse-light", [
      {
        line(node, lineNumber) {
          node.type = "element";
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;
          node.properties["data-line-type"] = lineType;
        },
      },
    ]);
    const pre = hast.children[0] as Element;
    const code = pre.children[0] as Element;
    const spans = code.children.filter(
      (child): child is Element => child.type === "element",
    );
    if (side === "left") return { kind: "deleted" as const };
    return { kind: "created" as const, spans };
  } else {
    return { kind: "no-change" as const };
  }
}
