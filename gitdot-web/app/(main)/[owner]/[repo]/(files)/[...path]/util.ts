import { structuredPatch } from "diff";
import type {
  RepositoryBlobResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Element, Root, Text } from "hast";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";

// ============================================================================
// folder trees
// ============================================================================

export type FolderTreeRowData = {
  name: string;
  path: string;
  isTree: boolean;
  isExpanded: boolean;
  fileCount: number;
  depth: number;
  isLast: boolean;
};

export function buildTreeRows(
  path: string,
  paths: RepositoryPathsResource,
  expandedPaths: Set<string>,
  depth = 0,
): FolderTreeRowData[] {
  const entries = getFolderEntries(path, paths);
  const lines: FolderTreeRowData[] = [];

  for (let i = 0; i < entries.length; i++) {
    const entry = entries[i];
    const name = entry.path.split("/").pop() ?? "";
    const isTree = entry.path_type === "tree";
    const isLast = i === entries.length - 1;
    const isExpanded = isTree && expandedPaths.has(entry.path);

    const fileCount = isTree
      ? paths.entries.filter((e) => {
          const prefix = `${entry.path}/`;
          return (
            e.path.startsWith(prefix) &&
            !e.path.slice(prefix.length).includes("/")
          );
        }).length
      : 0;
    lines.push({
      name,
      path: entry.path,
      isTree,
      isExpanded,
      fileCount,
      depth,
      isLast,
    });
    if (isExpanded) {
      lines.push(...buildTreeRows(entry.path, paths, expandedPaths, depth + 1));
    }
  }
  return lines;
}

export function expandPaths(
  path: string,
  paths: RepositoryPathsResource,
  depth = 1,
): Set<string> {
  const result = new Set<string>();
  const expand = (cur: string, d: number) => {
    if (d <= 0) return;
    for (const e of getFolderEntries(cur, paths)) {
      if (e.path_type === "tree") {
        result.add(e.path);
        expand(e.path, d - 1);
      }
    }
  };
  expand(path, depth);
  return result;
}

export type LineSelection = {
  start: number;
  end: number;
};

export function parseLineSelection(
  param: string | string[] | undefined,
): LineSelection | null {
  if (!param || typeof param !== "string") return null;
  if (param.includes("-")) {
    const [start, end] = param.split("-").map(Number);
    if (
      !Number.isNaN(start) &&
      !Number.isNaN(end) &&
      start > 0 &&
      end >= start
    ) {
      return { start, end };
    }
  } else {
    const line = Number(param);
    if (!Number.isNaN(line) && line > 0) {
      return { start: line, end: line };
    }
  }
  return null;
}

export function formatLineSelection(selection: LineSelection): string {
  return selection.start === selection.end
    ? `${selection.start}`
    : `${selection.start}-${selection.end}`;
}

// ============================================================================
// commit diffs
// ============================================================================

export function computeCommitDiffs(
  commits: Array<{ sha: string; parent_sha: string }>,
  blobs: RepositoryBlobResource[],
  hasts: Record<string, Root>,
): Record<string, Root> {
  const result: Record<string, Root> = {};

  for (let i = 0; i < commits.length; i++) {
    const commit = commits[i];
    const shaBlob = blobs.find((b) => b.commit_sha === commit.sha);
    if (!shaBlob || shaBlob.type !== "file") continue;

    const shaHast = hasts[commit.sha];
    if (!shaHast) continue;

    // use pairs of commit shas to determine deltas
    // this relies on the fact that though we may not find every commit for the file, we do not have gaps between commits
    // and if parent hast is missing (e.g., that file reference does not exist), we can safely say the file did not exist prior then
    const parentSha = i + 1 < commits.length ? commits[i + 1].sha : commit.parent_sha;
    const parentBlob = blobs.find((b) => b.commit_sha === parentSha && b.type === "file");
    const parentHast = parentSha ? hasts[parentSha] : undefined;

    result[commit.sha] = computeCommitDiff(parentBlob, parentHast, shaBlob, shaHast);
  }

  return result;
}

// unified diff algorithm
// for no hunk:
// - append from either and increment both cursors
//
// for a hunk:
// - lines match: append from either and increment both
// - left side only: append from left and increment left
// - right side only: append from right and increment right
function computeCommitDiff(
  leftBlob: RepositoryBlobResource | undefined,
  leftHast: Root | undefined,
  rightBlob: RepositoryBlobResource,
  rightHast: Root,
): Root {
  const rightLines = extractLines(rightHast);
  const leftLines = leftBlob && leftHast ? extractLines(leftHast) : [];

  const patch = structuredPatch(
    leftBlob?.path ?? rightBlob.path,
    rightBlob.path,
    leftBlob?.content ?? "",
    rightBlob.content,
    "",
    "",
    { context: 0 },
  );

  const resultLines: Element[] = [];
  let newCursor = 1;

  for (const hunk of patch.hunks) {
    while (newCursor < hunk.newStart) {
      const src = rightLines[newCursor - 1];
      if (src) resultLines.push(structuredClone(src));
      newCursor++;
    }

    let oldCursor = hunk.oldStart;
    for (const line of hunk.lines) {
      if (line.startsWith(" ")) {
        const src = rightLines[newCursor - 1];
        if (src) resultLines.push(structuredClone(src));
        oldCursor++;
        newCursor++;
      } else if (line.startsWith("-")) {
        const src = leftLines[oldCursor - 1];
        if (src) resultLines.push(colorBackground(src, "#fef2f2"));
        oldCursor++;
      } else if (line.startsWith("+")) {
        const src = rightLines[newCursor - 1];
        if (src) resultLines.push(colorBackground(src, "#f0fdf4"));
        newCursor++;
      }
    }
  }

  while (newCursor <= rightLines.length) {
    const src = rightLines[newCursor - 1];
    if (src) resultLines.push(structuredClone(src));
    newCursor++;
  }

  return buildInlineDiffHast(resultLines);
}

// shiki has puts lines of code as the type "element" interspersed with newlines
function buildInlineDiffHast(lines: Element[]): Root {
  const codeChildren: Array<Element | Text> = [];
  for (let i = 0; i < lines.length; i++) {
    codeChildren.push(lines[i]);
    if (i < lines.length - 1) {
      codeChildren.push({ type: "text", value: "\n" });
    }
  }
  return {
    type: "root",
    children: [
      {
        type: "element",
        tagName: "pre",
        properties: { class: ["outline-none"] },
        children: [
          {
            type: "element",
            tagName: "code",
            properties: { class: ["flex", "flex-col"] },
            children: codeChildren,
          },
        ],
      },
    ],
  };
}

// using explicit bg colors to avoid tailwind tree-shaking dependency
function colorBackground(line: Element, color: string): Element {
  const cloned = structuredClone(line);
  const existing = cloned.properties.style;
  cloned.properties.style = existing
    ? `${existing}; background-color: ${color}`
    : `background-color: ${color}`;
  return cloned;
}

function extractLines(hast: Root): Element[] {
  const pre = hast.children[0] as Element;
  const code = pre.children[0] as Element;
  return code.children.filter((c): c is Element => c.type === "element");
}
