"use client";

import type {
  RepositoryBlobsResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { useParams } from "next/navigation";
import { Fragment, useEffect, useRef, useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import { DatabaseProvider } from "@/provider/database";
import Link from "@/ui/link";
import { cn } from "@/util";
import { FolderTree } from "./folder-tree";

type TreeLine = {
  hasMoreSiblings: boolean[];
  isLast: boolean;
  name: string;
  path: string;
  isTree: boolean;
  isExpanded: boolean;
  depth: number;
};

function _flattenTree(
  folderPath: string,
  paths: RepositoryPathsResource,
  expandedPaths: Set<string>,
  hasMoreSiblings: boolean[] = [],
  depth = 0,
): TreeLine[] {
  const entries = getFolderEntries(folderPath, paths);
  const lines: TreeLine[] = [];
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
        ..._flattenTree(
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

function flattenAll(
  folderPath: string,
  paths: RepositoryPathsResource,
  hasMoreSiblings: boolean[] = [],
  depth = 0,
  maxDepth = Number.POSITIVE_INFINITY,
): TreeLine[] {
  const entries = getFolderEntries(folderPath, paths);
  const lines: TreeLine[] = [];
  for (let i = 0; i < entries.length; i++) {
    const entry = entries[i];
    const name = entry.path.split("/").pop()!;
    const isTree = entry.path_type === "tree";
    const isLast = i === entries.length - 1;
    lines.push({
      hasMoreSiblings,
      isLast,
      name,
      path: entry.path,
      isTree,
      isExpanded: false,
      depth,
    });
    if (isTree && depth + 1 < maxDepth) {
      lines.push(
        ...flattenAll(
          entry.path,
          paths,
          [...hasMoreSiblings, !isLast],
          depth + 1,
          maxDepth,
        ),
      );
    }
  }
  return lines;
}

function TreeLinePrefix({
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
            <span className="absolute left-[9px] top-0 bottom-0 border-l border-foreground" />
          )}
        </span>
      ))}
      <span className="relative w-5">
        <span
          className={cn(
            "absolute left-[9px] border-l border-foreground",
            isLast ? "top-0 bottom-1/2" : "top-0 bottom-0",
          )}
        />
        <span className="absolute left-[9px] right-0 top-1/2 border-t border-foreground" />
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
  folderPath,
  paths,
  owner,
  repo,
}: {
  folderPath: string;
  paths: RepositoryPathsResource;
  owner: string;
  repo: string;
}) {
  const prefix = folderPath ? `${folderPath}/` : "";
  const under = paths.entries.filter(
    (e) => e.path.startsWith(prefix) && e.path !== folderPath,
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
        {folderPath?.split("/").map((seg, i, arr) => (
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

function TreeRows({
  lines,
  paths,
  blobs,
  owner,
  repo,
  showAbsolutePath = false,
  onToggle,
  onMouseEnter,
}: {
  lines: TreeLine[];
  paths: RepositoryPathsResource;
  blobs: RepositoryBlobsResource | null;
  owner: string;
  repo: string;
  showAbsolutePath?: boolean;
  onToggle?: (path: string) => void;
  onMouseEnter?: (path: string, isTree: boolean) => void;
}) {
  return (
    <>
      {lines.map((line) =>
        line.isTree ? (
          // biome-ignore lint/a11y/useKeyWithClickEvents: expand/collapse on click
          <div
            key={line.path}
            className="flex items-stretch gap-1.5 font-mono text-sm h-6 shrink-0 select-none hover:bg-accent w-full pl-1 pr-2"
            onMouseEnter={() => onMouseEnter?.(line.path, true)}
            onClick={() => onToggle?.(line.path)}
          >
            <TreeLinePrefix
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
            onMouseEnter={() => onMouseEnter?.(line.path, false)}
          >
            <TreeLinePrefix
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
    </>
  );
}

type Preview =
  | { kind: "file"; hast: Root }
  | { kind: "folder"; path: string; lines: TreeLine[] };

export function FolderViewer({ path }: { path: string }) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [paths, setPaths] = useState<RepositoryPathsResource | null>(null);
  const [blobs, setBlobs] = useState<RepositoryBlobsResource | null>(null);
  const [preview, setPreview] = useState<Preview | null>(null);
  const [previewPath, setPreviewPath] = useState<string | null>(null);
  const dbRef = useRef<DatabaseProvider | null>(null);
  const pathsRef = useRef<RepositoryPathsResource | null>(null);

  useEffect(() => {
    const db = new DatabaseProvider(owner, repo);
    dbRef.current = db;
    db.getPaths().then((p) => {
      if (!p) return;
      setPaths(p);
      pathsRef.current = p;
      const _topLevel = getFolderEntries(path ?? "", p);
    });
    db.getBlobs().then((b) => {
      if (b) setBlobs(b);
    });
  }, [owner, repo, path]);

  const _handleHover = (path: string, isTree: boolean) => {
    if (isTree) {
      if (pathsRef.current) {
        setPreview({
          kind: "folder",
          path,
          lines: flattenAll(path, pathsRef.current, [], 0, 2),
        });
      }
      return;
    }
    dbRef.current?.getHast(path).then((hast) => {
      if (hast) setPreview({ kind: "file", hast });
      else setPreview(null);
    });
  };

  if (!paths) return null;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <FolderTree
        path={path}
        owner={owner}
        repo={repo}
        paths={paths}
        blobs={blobs}
        setPreview={setPreviewPath}
      />
      <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
        {preview?.kind === "file" && (
          <div className="text-sm px-2 py-1.5">
            {
              toJsxRuntime(preview.hast, {
                Fragment,
                jsx,
                jsxs,
              }) as React.JSX.Element
            }
          </div>
        )}
        {preview?.kind === "folder" && (
          <div className="flex flex-col">
            <TreeHeader
              folderPath={preview.path}
              paths={paths}
              owner={owner}
              repo={repo}
            />
            <TreeRows
              lines={preview.lines}
              paths={paths}
              blobs={blobs}
              owner={owner}
              repo={repo}
              showAbsolutePath
            />
          </div>
        )}
      </div>
    </div>
  );
}
