import type { RepositoryBlobResource, RepositoryPathsResource } from "gitdot-api";
import type { Element, Root, Text } from "hast";
import { structuredPatch } from "diff";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";

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
// inline diff
// ============================================================================

export function computeInlineDiffs(
  currentSha: string,
  blobs: RepositoryBlobResource[],
  hasts: Record<string, Root>,
): Record<string, Root> {
  const currentBlob = blobs.find((b) => b.commit_sha === currentSha);
  if (!currentBlob || currentBlob.type !== "file") return {};

  const currentHast = hasts[currentSha];
  if (!currentHast) return {};

  const currentLines = extractLines(currentHast);
  const result: Record<string, Root> = {};

  for (const otherBlob of blobs) {
    if (otherBlob.type !== "file") continue;
    if (otherBlob.commit_sha === currentBlob.commit_sha) continue;

    const otherHast = hasts[otherBlob.commit_sha];
    if (!otherHast) continue;

    const otherLines = extractLines(otherHast);
    const patch = structuredPatch(
      otherBlob.path,
      currentBlob.path,
      otherBlob.content,
      currentBlob.content,
      "",
      "",
      { context: 0 },
    );
    console.log(patch);
    
    const resultLines: Element[] = [];
    let oldCursor = 1;

    for (const hunk of patch.hunks) {
      while (oldCursor < hunk.oldStart) {
        const src = otherLines[oldCursor - 1];
        if (src) resultLines.push(structuredClone(src));
        oldCursor++;
      }

      let newLineCursor = hunk.newStart;
      for (const line of hunk.lines) {
        if (line.startsWith(" ")) {
          const src = otherLines[oldCursor - 1];
          if (src) resultLines.push(structuredClone(src));
          oldCursor++;
          newLineCursor++;
        } else if (line.startsWith("-")) {
          const src = otherLines[oldCursor - 1];
          if (src) resultLines.push(colorBackground(src, "#fef2f2"));
          oldCursor++;
        } else if (line.startsWith("+")) {
          const src = currentLines[newLineCursor - 1];
          if (src) resultLines.push(colorBackground(src, "#f0fdf4"));
          newLineCursor++;
        }
        // skip "\" no-newline-at-EOF markers
      }
    }

    while (oldCursor <= otherLines.length) {
      const src = otherLines[oldCursor - 1];
      if (src) resultLines.push(structuredClone(src));
      oldCursor++;
    }

    result[otherBlob.commit_sha] = buildInlineDiffHast(resultLines);
  }

  return result;
}

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
