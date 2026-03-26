"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment, useEffect, useRef, useState } from "react";
import { useRouter } from "next/navigation";
import { jsx, jsxs } from "react/jsx-runtime";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { Loading } from "@/ui/loading";
import { expandPaths, buildTreeRows } from "../util";
import { FolderTreeRow } from "./folder-tree-row";

export function FolderPathPreview({
  previewPath,
  paths,
  owner,
  repo,
  getHast,
}: {
  previewPath: string | null;
  paths: RepositoryPathsResource;
  owner: string;
  repo: string;
  getHast: (path: string) => Promise<Root | null>;
}) {
  const entry = previewPath
    ? paths.entries.find((e) => e.path === previewPath)
    : null;
  if (!previewPath || !entry) return null;

  if (entry.path_type === "tree") {
    return <FolderTreePreview path={previewPath} paths={paths} owner={owner} repo={repo} />
  } else {
    return (
      <FilePreview path={previewPath} getHast={getHast} />
    );
  }
}

export function FolderTreePreview({
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
  const [hoveredPath, setHoveredPath] = useState<string | null>(null);

  const router = useRouter();
  const mouseMoved = useRef(false);
  const expandedPaths = expandPaths(path, paths, Number.POSITIVE_INFINITY);

  return (
    <div
      className="flex-1 min-w-0 overflow-auto scrollbar-thin"
      onMouseMove={() => { mouseMoved.current = true; }}
    >
      <div className="flex flex-col">
        {getFolderEntries(path, paths)
          .filter((e) => e.path_type === "tree")
          .map((subfolder) => (
            <FolderPreview
            key={subfolder.path}
            entryPath={subfolder.path}
            owner={owner}
            repo={repo}
            rows={buildTreeRows(subfolder.path, paths, expandedPaths)}
            hoveredPath={hoveredPath}
            onMouseEnter={(path) => {
              if (!mouseMoved.current) return;
              setHoveredPath(path);
            }}
            onNavigate={(path) => router.push(`/${owner}/${repo}/${path}`)}
            />
          ))}
      </div>
    </div>
  );
}

function FolderPreview({
  entryPath,
  owner,
  repo,
  rows,
  hoveredPath,
  onMouseEnter,
  onNavigate,
}: {
  entryPath: string;
  owner: string;
  repo: string;
  rows: ReturnType<typeof buildTreeRows>;
  hoveredPath: string | null;
  onMouseEnter: (path: string) => void;
  onNavigate: (path: string) => void;
}) {
  const name = entryPath.split("/").pop() ?? "";
  const prefix = entryPath.split("/").slice(0, -1).join("/");

  return (
    <div>
      <div className="flex items-center font-mono text-sm h-6 shrink-0 select-none pl-2.5">
        <Link
          href={`/${owner}/${repo}/${entryPath}`}
          className="flex items-center cursor-pointer hover:underline"
        >
          <span className="text-muted-foreground">{prefix}/</span>
          {name}/
        </Link>
      </div>
      {rows.map((row) => (
        <FolderTreeRow
          key={row.path}
          row={row}
          owner={owner}
          repo={repo}
          absolutePaths={true}
          focused={hoveredPath === row.path}
          onMouseEnter={() => onMouseEnter(row.path)}
          onClick={onNavigate}
        />
      ))}
    </div>
  );
}

function FilePreview({
  path,
  getHast,
}: {
  path: string;
  getHast: (path: string) => Promise<Root | null>;
}) {
  const [hast, setHast] = useState<Root | null>(null);

  useEffect(() => {
    setHast(null);
    getHast(path).then(setHast);
  }, [path, getHast]);

  return <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
    {
      hast ? (
        <div className="text-sm px-2 py-1.5">
          {toJsxRuntime(hast, { Fragment, jsx, jsxs }) as React.JSX.Element}
        </div>
      ) : (
        <Loading />
      )
    }
  </div>
}
