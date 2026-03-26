"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment, useEffect, useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { FolderTree } from "./folder-tree";

export function FolderTreePreview({
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
  const [hast, setHast] = useState<Root | null>(null);

  const entry = previewPath
    ? paths.entries.find((e) => e.path === previewPath)
    : null;
  const isTree = entry?.path_type === "tree";

  useEffect(() => {
    if (!previewPath || isTree) {
      setHast(null);
      return;
    }
    getHast(previewPath).then(setHast);
  }, [previewPath, isTree, getHast]);

  if (!previewPath || !entry) return null;

  return (
    <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
      {isTree ? (
        <FolderTree
          path={previewPath}
          paths={paths}
          owner={owner}
          repo={repo}
          absolutePaths={true}
        />
      ) : hast ? (
        <div className="text-sm px-2 py-1.5">
          {toJsxRuntime(hast, { Fragment, jsx, jsxs }) as React.JSX.Element}
        </div>
      ) : null}
    </div>
  );
}
