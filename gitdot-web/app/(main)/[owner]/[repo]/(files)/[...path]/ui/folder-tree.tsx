"use client";

import type {
  RepositoryBlobsResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { useState } from "react";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { cn } from "@/util";

type TreeRowData = {
  hasMoreSiblings: boolean[];
  isLast: boolean;
  name: string;
  path: string;
  isTree: boolean;
  isExpanded: boolean;
  depth: number;
};

function flattenTree(
  path: string,
  paths: RepositoryPathsResource,
  expandedPaths: Set<string>,
  hasMoreSiblings: boolean[] = [],
  depth = 0,
): TreeRowData[] {
  const entries = getFolderEntries(path, paths);
  const lines: TreeRowData[] = [];

  for (let i = 0; i < entries.length; i++) {
    const entry = entries[i];
    const name = entry.path.split("/").pop()!;
    const isTree = entry.path_type === "tree";
    const isLast = i === entries.length - 1;
    const isExpanded = isTree && expandedPaths.has(entry.path);

    lines.push({
      hasMoreSiblings,
      isLast,
      name,
      path: entry.path,
      isTree,
      isExpanded,
      depth,
    });
    if (isExpanded) {
      lines.push(
        ...flattenTree(
          entry.path,
          paths,
          expandedPaths,
          [...hasMoreSiblings, !isLast],
          depth + 1,
        ),
      );
    }
  }
  return lines;
}

function TreeRowGutter({
  hasMoreSiblings,
  isLast,
}: {
  hasMoreSiblings: boolean[];
  isLast: boolean;
}) {
  return (
    <span className="flex items-stretch shrink-0 select-none" aria-hidden>
      {hasMoreSiblings.map((active, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: stable positional slots
        <span key={i} className="relative w-5">
          {active && (
            <span className="absolute left-2.25 top-0 bottom-0 border-l border-foreground" />
          )}
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

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
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
        <Link href={`/${owner}/${repo}`} className="hover:underline">
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
      </div>
      <span className="text-xs text-muted-foreground">{fileCount} files</span>
    </div>
  );
}

export function FolderTree({
  path,
  paths,
  blobs,
  owner,
  repo,
  showAbsolutePath = false,
}: {
  path: string;
  paths: RepositoryPathsResource;
  blobs: RepositoryBlobsResource | null;
  owner: string;
  repo: string;
  showAbsolutePath?: boolean;
}) {
  const [_expandedPaths, setExpandedPaths] = useState<Set<string>>(new Set());

  const _toggleFolder = (path: string) => {
    setExpandedPaths((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  };

  const lines = flattenTree(path ?? "", paths, new Set());

  return (
    <div
      data-page-scroll
      className="flex flex-col w-[45%] h-full shrink-0 border-r overflow-y-auto scrollbar-thin"
    >
      <TreeHeader path={path ?? ""} paths={paths} owner={owner} repo={repo} />
      {lines.map((line) =>
        line.isTree ? (
          // biome-ignore lint/a11y/useKeyWithClickEvents: expand/collapse on click
          <div
            key={line.path}
            className="flex items-stretch gap-1.5 font-mono text-sm h-6 shrink-0 select-none hover:bg-accent w-full pl-1 pr-2"
            // onMouseEnter={() => onMouseEnter?.(line.path, true)}
            // onClick={() => onToggle?.(line.path)}
          >
            <TreeRowGutter
              hasMoreSiblings={line.hasMoreSiblings}
              isLast={line.isLast}
            />
            <Link
              href={`/${owner}/${repo}/${line.path}`}
              className="flex items-center cursor-pointer hover:underline"
              onClick={(e) => e.stopPropagation()}
            >
              {showAbsolutePath && (
                <span className="text-muted-foreground">
                  {line.path.split("/").slice(0, -1).join("/")}/
                </span>
              )}
              {line.name}/
            </Link>
            <span className="text-xs text-muted-foreground ml-auto flex items-center">
              {
                paths.entries.filter(
                  (e) =>
                    e.path.startsWith(`${line.path}/`) &&
                    e.path_type === "blob",
                ).length
              }{" "}
              files
            </span>
          </div>
        ) : (
          <Link
            key={line.path}
            href={`/${owner}/${repo}/${line.path}`}
            className="flex items-stretch gap-1.5 font-mono text-sm h-6 shrink-0 select-none hover:bg-accent cursor-default px-1"
            // onMouseEnter={() => onMouseEnter?.(line.path, false)}
          >
            <TreeRowGutter
              hasMoreSiblings={line.hasMoreSiblings}
              isLast={line.isLast}
            />
            <span className="flex items-center">
              {showAbsolutePath && (
                <span className="text-muted-foreground">
                  {line.path.split("/").slice(0, -1).join("/")}/
                </span>
              )}
              {line.name}
            </span>
            {blobs &&
              (() => {
                const blob = blobs.blobs.find(
                  (b) => b.type === "file" && b.path === line.path,
                );
                return blob && blob.type === "file" ? (
                  <span className="text-xs text-muted-foreground ml-auto flex items-center pr-1">
                    {formatBytes(new TextEncoder().encode(blob.content).length)}
                  </span>
                ) : null;
              })()}
          </Link>
        ),
      )}
    </div>
  );
}
