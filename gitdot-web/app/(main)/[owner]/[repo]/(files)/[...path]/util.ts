import type { RepositoryPathsResource } from "gitdot-api";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";

export type FolderTreeRow = {
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
): FolderTreeRow[] {
  const entries = getFolderEntries(path, paths);
  const lines: FolderTreeRow[] = [];

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
