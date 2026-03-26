"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useMemo, useState } from "react";
import { DatabaseProvider } from "@/provider/database";
import { FolderTree } from "./folder-tree";
import { FolderPathPreview } from "./folder-path-preview";

export function FolderViewer({
  path,
  paths,
  initialExpanded,
  activePath,
}: {
  path: string;
  paths: RepositoryPathsResource | null;
  initialExpanded?: Set<string>;
  activePath?: string;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [previewPath, setPreviewPath] = useState<string | null>(null);
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
          previewPath={previewPath}
          setPreviewPath={setPreviewPath}
          initialExpanded={initialExpanded}
          activePath={activePath}
        />
      </div>
      <FolderPathPreview
        previewPath={previewPath}
        paths={paths}
        owner={owner}
        repo={repo}
        getHast={getHast}
      />
    </div>
  );
}
