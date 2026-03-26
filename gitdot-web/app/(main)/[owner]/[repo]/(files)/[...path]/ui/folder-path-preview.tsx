"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment, useEffect, useRef, useState } from "react";
import { useRouter } from "next/navigation";
import { jsx, jsxs } from "react/jsx-runtime";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { cn } from "@/util";
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
    return <FolderPreview path={previewPath} paths={paths} owner={owner} repo={repo} />
  } else {
    return (
      <FilePreview path={previewPath} getHast={getHast} />
    );
  }
}

function FolderPreview({
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
        {getFolderEntries(path, paths).map((entry) => {
            const name = entry.path.split("/").pop() ?? "";
            const prefix = entry.path.split("/").slice(0, -1).join("/");
            const navigate = (p: string) => router.push(`/${owner}/${repo}/${p}`);
            const onMouseEnter = () => {
              if (!mouseMoved.current) return;
              setHoveredPath(entry.path);
            };

            if (entry.path_type === "tree") {
              const rows = buildTreeRows(entry.path, paths, expandedPaths);
              return (
                <div key={entry.path}>
                  <div
                    className={cn("flex items-center font-mono text-sm h-6 shrink-0 select-none pl-2.5", hoveredPath === entry.path && "bg-accent/50")}
                    onMouseEnter={onMouseEnter}
                  >
                    <Link
                      href={`/${owner}/${repo}/${entry.path}`}
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
                      onMouseEnter={() => {
                        if (!mouseMoved.current) return;
                        setHoveredPath(row.path);
                      }}
                      onClick={navigate}
                    />
                  ))}
                </div>
              );
            }

            return (
              <div
                key={entry.path}
                className={cn("flex items-center font-mono text-sm h-6 shrink-0 select-none pl-2.5", hoveredPath === entry.path && "bg-accent/50")}
                onMouseEnter={onMouseEnter}
              >
                <Link
                  href={`/${owner}/${repo}/${entry.path}`}
                  data-path={entry.path}
                  className="flex items-center cursor-default"
                >
                  <span className="text-muted-foreground">{prefix}/</span>
                  {name}
                </Link>
              </div>
            );
          })}
      </div>
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
