"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment, useEffect, useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { Loading } from "@/ui/loading";
import { useFolderViewerContext } from "./folder-viewer-context";

export function FolderPathPreview({
  paths,
  owner,
  repo,
  getHast,
}: {
  paths: RepositoryPathsResource;
  owner: string;
  repo: string;
  getHast: (path: string) => Promise<Root | null>;
}) {
  const { previewPath } = useFolderViewerContext();
  const entry = previewPath
    ? paths.entries.find((e) => e.path === previewPath)
    : null;
  if (!previewPath || !entry) return <PreviewPlaceholder />;

  if (entry.path_type === "tree") {
    return (
      <FolderPreview
        path={previewPath}
        paths={paths}
        owner={owner}
        repo={repo}
      />
    );
  } else {
    return <FilePreview path={previewPath} getHast={getHast} />;
  }
}

function PreviewPlaceholder() {
  return (
    <div className="flex-1 min-w-0 flex flex-col items-center justify-center h-full w-full pb-[5%]">
      <span className="font-mono text-sm text-muted-foreground lowercase">
        select a file to preview...
      </span>
    </div>
  );
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
  return (
    <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
      <div className="flex flex-col">
        {getFolderEntries(path, paths).map((entry) => {
          const name = entry.path.split("/").pop() ?? "";
          const prefix = entry.path.split("/").slice(0, -1).join("/");

          return (
            <div
              key={entry.path}
              className="flex items-center font-mono text-sm h-6 shrink-0 select-none pl-2.5"
            >
              <Link
                href={`/${owner}/${repo}/${entry.path}`}
                className="flex items-center cursor-pointer hover:underline"
              >
                <span className="text-muted-foreground">{prefix}/</span>
                {name}
                {entry.path_type === "tree" && "/"}
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

  return (
    <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
      {hast ? (
        <div className="text-sm px-2 py-1.5">
          {toJsxRuntime(hast, { Fragment, jsx, jsxs }) as React.JSX.Element}
        </div>
      ) : (
        <Loading />
      )}
    </div>
  );
}
