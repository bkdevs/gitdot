"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useMemo, useState } from "react";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import { DatabaseProvider } from "@/provider/database";
import { FolderTree } from "./folder-tree";
import { FolderTreePreview } from "./folder-tree-preview";

export function FolderViewer({
  path,
  paths,
}: {
  path: string;
  paths: RepositoryPathsResource | null;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [hoveredPath, setHoveredPath] = useState<string | null>(null);
  const [pinnedPath, setPinnedPath] = useState<string | null>(() => {
    if (!paths) return null;
    const entries = getFolderEntries(path, paths);
    return entries[0]?.path ?? null;
  });
  const db = useMemo(() => new DatabaseProvider(owner, repo), [owner, repo]);

  const getHast = (p: string): Promise<Root | null> => db.getHast(p);

  const handleHover = (path: string) => {
    setHoveredPath(path);
  };

  const handlePin = (path: string) => {
    setPinnedPath(path);
    setHoveredPath(path);
  };

  if (!paths) return null;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div className="w-[45%] shrink-0 border-r h-full">
        <FolderTree
          path={path}
          owner={owner}
          repo={repo}
          paths={paths}
          shortcuts
          onHover={handleHover}
          onHoverClear={() => setHoveredPath(null)}
          onPin={handlePin}
          pinnedPath={pinnedPath}
          absolutePaths={false}
        />
      </div>
      <FolderTreePreview
        previewPath={hoveredPath ?? pinnedPath}
        paths={paths}
        owner={owner}
        repo={repo}
        getHast={getHast}
      />
    </div>
  );
}
