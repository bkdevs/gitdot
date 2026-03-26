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
  setPreview,
}: {
  owner: string;
  repo: string;
  path: string;
  paths: RepositoryPathsResource;
  absolutePaths: boolean;
  setPreview?: (path: string) => void;
}) {
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(() => {
    const entries = getFolderEntries(path, paths);
    return new Set(
      entries.filter((e) => e.path_type === "tree").map((e) => e.path),
    );
  });

  const toggleFolder = (path: string) => {
    setExpandedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  };

  const rows = buildRows(path ?? "", paths, expandedPaths);

  return (
    <div
      data-page-scroll
      className="flex flex-col h-full overflow-y-auto scrollbar-thin"
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
            setPreview={setPreview}
            onToggle={toggleFolder}
          />
        ) : (
          <TreeRowFile
            key={row.path}
            row={row}
            owner={owner}
            repo={repo}
            setPreview={setPreview}
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
        {path?.split("/").map((seg, i, arr) => (
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
  setPreview,
  absolutePaths,
  onToggle,
}: {
  row: TreeRowData;
  owner: string;
  repo: string;
  setPreview?: (path: string) => void;
  absolutePaths: boolean;
  onToggle: (path: string) => void;
}) {
  return (
    <button
      type="button"
      className="flex items-stretch gap-1.5 font-mono text-sm h-6 shrink-0 select-none hover:bg-accent w-full pl-1 pr-2"
      onMouseEnter={() => setPreview?.(row.path)}
      onClick={() => onToggle(row.path)}
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
  owner,
  repo,
  setPreview,
  absolutePaths,
}: {
  row: TreeRowData;
  owner: string;
  repo: string;
  setPreview?: (path: string) => void;
  absolutePaths: boolean;
}) {
  return (
    <Link
      key={row.path}
      href={`/${owner}/${repo}/${row.path}`}
      className="flex items-stretch gap-1.5 font-mono text-sm h-6 shrink-0 select-none hover:bg-accent cursor-default px-1"
      onMouseEnter={() => setPreview?.(row.path)}
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
    </Link>
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
        <span className="absolute left-2.25 right-0 top-1/2 border-t border-foreground" />
      </span>
    </span>
  );
}
