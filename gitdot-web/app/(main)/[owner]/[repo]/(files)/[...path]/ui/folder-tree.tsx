"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import { useState } from "react";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { cn } from "@/util";

type TreeRowData = {
  name: string;
  path: string;
  isTree: boolean;
  isExpanded: boolean;
  fileCount: number;
  depth: number;
  isLast: boolean;
};

function buildRows(
  path: string,
  paths: RepositoryPathsResource,
  expandedPaths: Set<string>,
  depth = 0,
): TreeRowData[] {
  const entries = getFolderEntries(path, paths);
  const lines: TreeRowData[] = [];

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
    lines.push({ name, path: entry.path, isTree, isExpanded, fileCount, depth, isLast });
    if (isExpanded) {
      lines.push(...buildRows(entry.path, paths, expandedPaths, depth + 1));
    }
  }
  return lines;
}

export function FolderTree({
  owner,
  repo,
  path,
  paths,
  absolutePaths = false,
  onHover,
  onHoverClear,
  onPin,
  pinnedPath,
}: {
  owner: string;
  repo: string;
  path: string;
  paths: RepositoryPathsResource;
  absolutePaths: boolean;
  onHover?: (path: string) => void;
  onHoverClear?: () => void;
  onPin?: (path: string) => void;
  pinnedPath?: string | null;
}) {
  const [focusedPath, setFocusedPath] = useState<string | null>(null);
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(() => {
    const entries = getFolderEntries(path, paths);
    return new Set(
      entries.filter((e) => e.path_type === "tree").map((e) => e.path),
    );
  });

  const toggleFolder = (path: string) => {
    setExpandedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(path)) {
        for (const p of next) {
          if (p === path || p.startsWith(`${path}/`)) next.delete(p);
        }
      } else {
        next.add(path);
      }
      return next;
    });
  };

  const rows = buildRows(path ?? "", paths, expandedPaths);

  return (
    <div
      data-page-scroll
      className="flex flex-col h-full overflow-y-auto scrollbar-thin"
      onMouseLeave={() => { setFocusedPath(null); onHoverClear?.(); }}
      onMouseOver={(e) => { if (e.target === e.currentTarget) { setFocusedPath(null); onHoverClear?.(); } }}
    >
      <TreeHeader path={path} paths={paths} owner={owner} repo={repo} />
      {rows.map((row) =>
        row.isTree ? (
          <TreeRowFolder
            key={row.path}
            row={row}
            owner={owner}
            repo={repo}
            absolutePaths={absolutePaths}
            focused={focusedPath === row.path}
            pinned={pinnedPath === row.path}
            onMouseEnter={() => { setFocusedPath(row.path); onHover?.(row.path); }}
            onToggle={toggleFolder}
            onPin={onPin}
          />
        ) : (
          <TreeRowFile
            key={row.path}
            row={row}
            focused={focusedPath === row.path}
            pinned={pinnedPath === row.path}
            onMouseEnter={() => { setFocusedPath(row.path); onHover?.(row.path); }}
            onPin={onPin}
            absolutePaths={absolutePaths}
          />
        ),
      )}
    </div>
  );
}

function TreeHeader({
  path,
  paths,
  owner,
  repo,
}: {
  path: string;
  paths: RepositoryPathsResource;
  owner: string;
  repo: string;
}) {
  const prefix = path ? `${path}/` : "";
  const under = paths.entries.filter(
    (e) => e.path.startsWith(prefix) && e.path !== path,
  );
  const fileCount = under.filter((e) => e.path_type === "blob").length;

  return (
    <div className="font-mono text-sm h-6 flex items-center justify-between pl-3 pr-2 pt-2 shrink-0">
      <div className="flex items-center">
        <Link href={`/${owner}`} className="hover:underline">
          {owner}
        </Link>
        <span>/</span>
        <Link href={`/${owner}/${repo}/files`} className="hover:underline">
          {repo}
        </Link>
        {path?.length > 0 &&
          path?.split("/").map((seg, i, arr) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: stable path segments
            <span key={i}>
              <span>/</span>
              <Link
                href={`/${owner}/${repo}/${arr.slice(0, i + 1).join("/")}`}
                className="hover:underline"
              >
                {seg}
              </Link>
            </span>
          ))}
        <span>/</span>
      </div>
      <span className="text-xs text-muted-foreground">{fileCount} files</span>
    </div>
  );
}

function TreeRowFolder({
  row,
  owner,
  repo,
  focused,
  pinned,
  onMouseEnter,
  absolutePaths,
  onToggle,
  onPin,
}: {
  row: TreeRowData;
  owner: string;
  repo: string;
  focused: boolean;
  pinned: boolean;
  onMouseEnter: () => void;
  absolutePaths: boolean;
  onToggle: (path: string) => void;
  onPin?: (path: string) => void;
}) {
  return (
    <button
      type="button"
      className={cn("flex items-stretch gap-1 font-mono text-sm h-6 shrink-0 select-none w-full pl-1 pr-2", focused && "bg-accent", pinned && "underline")}
      onMouseEnter={onMouseEnter}
      onClick={() => { onToggle(row.path); onPin?.(row.path); }}
    >
      <TreeRowGutter depth={row.depth} isLast={row.isLast} />
      <Link
        href={`/${owner}/${repo}/${row.path}`}
        className="flex items-center cursor-pointer hover:underline"
        onClick={(e) => e.stopPropagation()}
      >
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        {row.name}
        {row.isExpanded && "/"}
        {!row.isExpanded && (
          <span className="ml-1 text-xs text-muted-foreground inline-flex items-center">
            ({row.fileCount})
          </span>
        )}
      </Link>
    </button>
  );
}

function TreeRowFile({
  row,
  focused,
  pinned,
  onMouseEnter,
  onPin,
  absolutePaths,
}: {
  row: TreeRowData;
  focused: boolean;
  pinned: boolean;
  onMouseEnter: () => void;
  onPin?: (path: string) => void;
  absolutePaths: boolean;
}) {
  return (
    <button
      type="button"
      className={cn("flex items-stretch gap-1 font-mono text-sm h-6 shrink-0 select-none cursor-default px-1 w-full", focused && "bg-accent", pinned && "underline")}
      onMouseEnter={onMouseEnter}
      onClick={() => onPin?.(row.path)}
    >
      <TreeRowGutter depth={row.depth} isLast={row.isLast} />
      <span className="flex items-center">
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        {row.name}
      </span>
    </button>
  );
}

function TreeRowGutter({ depth, isLast }: { depth: number; isLast: boolean }) {
  return (
    <span className="flex items-stretch shrink-0 select-none" aria-hidden>
      {Array.from({ length: depth }, (_, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: stable positional slots
        <span key={i} className="relative w-5">
          <span className="absolute left-2.25 top-0 bottom-0 border-l border-foreground" />
        </span>
      ))}
      <span className="relative w-5">
        <span
          className={cn(
            "absolute left-2.25 border-l border-foreground",
            isLast ? "top-0 bottom-1/2" : "top-0 bottom-0",
          )}
        />
        <span className="absolute left-2.25 right-0.5 top-1/2 border-t border-foreground" />
      </span>
    </span>
  );
}
