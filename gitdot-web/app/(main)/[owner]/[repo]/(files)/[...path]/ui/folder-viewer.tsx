"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useMemo, useState } from "react";
import { DatabaseProvider } from "@/provider/database";
import { FolderPathPreview } from "./folder-path-preview";
import { FolderTree } from "./folder-tree";

export function FolderViewer({
  path,
  paths,
}: {
  path: string;
  paths: RepositoryPathsResource | null;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [hoveredPath, setHoveredPath] = useState<string | null>(null);
  const [pinnedPath, setPinnedPath] = useState<string | null>(null);
  const db = useMemo(() => new DatabaseProvider(owner, repo), [owner, repo]);

  const getHast = (p: string): Promise<Root | null> => db.getHast(p);

  if (!paths) return null;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div className="w-[45%] shrink-0 border-r h-full">
        <FolderTree
          path={path}
          owner={owner}
          repo={repo}
          paths={paths}
          hoveredPath={hoveredPath}
          setHoveredPath={setHoveredPath}
          pinnedPath={pinnedPath}
          setPinnedPath={setPinnedPath}
        />
      </div>
      <FolderPathPreview
        previewPath={pinnedPath ?? hoveredPath}
        paths={paths}
        owner={owner}
        repo={repo}
        getHast={getHast}
      />
    </div>
  );
}
