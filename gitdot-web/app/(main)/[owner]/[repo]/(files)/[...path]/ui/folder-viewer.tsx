"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useMemo } from "react";
import { DatabaseProvider } from "@/provider/database";
import { FolderPathPreview } from "./folder-path-preview";
import { FolderTree } from "./folder-tree";
import { FolderViewerProvider } from "./folder-viewer-context";

export function FolderViewer({
  path,
  paths,
  pinFiles = true,
}: {
  path: string;
  paths: RepositoryPathsResource | null;
  pinFiles?: boolean;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const db = useMemo(() => new DatabaseProvider(owner, repo), [owner, repo]);

  const getHast = (p: string): Promise<Root | null> => db.getHast(p);

  if (!paths) return null;

  return (
    <FolderViewerProvider pinFiles={pinFiles}>
      <div className="flex w-full h-full min-h-0 overflow-hidden">
        <div className="w-[45%] shrink-0 border-r h-full">
          <FolderTree path={path} owner={owner} repo={repo} paths={paths} />
        </div>
        <FolderPathPreview
          paths={paths}
          owner={owner}
          repo={repo}
          getHast={getHast}
        />
      </div>
    </FolderViewerProvider>
  );
}
