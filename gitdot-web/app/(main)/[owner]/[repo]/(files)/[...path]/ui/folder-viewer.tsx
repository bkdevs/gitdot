"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useMemo, useState } from "react";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import { DatabaseProvider } from "@/provider/database";
import { FolderTree } from "./folder-tree";
import { FolderPathPreview } from "./folder-path-preview";

export function FolderViewer({
  path,
  paths,
}: {
  path: string;
  paths: RepositoryPathsResource | null;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [previewPath, setPreviewPath] = useState<string | null>(() => {
    if (!paths) return null;
    const entries = getFolderEntries(path, paths);
    return entries[0]?.path ?? null;
  });
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
