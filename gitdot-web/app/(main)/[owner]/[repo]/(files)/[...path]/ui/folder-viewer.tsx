"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import { DatabaseProvider } from "@/provider/database";
import { FolderTree } from "./folder-tree";
import { FolderTreePreview } from "./folder-tree-preview";

export function FolderViewer({ folderPath: path }: { folderPath: string }) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const [paths, setPaths] = useState<RepositoryPathsResource | null>(null);
  const [previewPath, setPreviewPath] = useState<string | null>(null);
  const dbRef = useRef<DatabaseProvider | null>(null);

  useEffect(() => {
    const db = new DatabaseProvider(owner, repo);
    dbRef.current = db;
    db.getPaths().then((p) => {
      if (p) setPaths(p);
    });
  }, [owner, repo]);

  const getHast = (path: string): Promise<Root | null> =>
    dbRef.current?.getHast(path) ?? Promise.resolve(null);

  if (!paths) return null;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div className="w-[45%] shrink-0 border-r h-full">
        <FolderTree
          path={path}
          owner={owner}
          repo={repo}
          paths={paths}
          setPreview={setPreviewPath}
          absolutePaths={false}
        />
      </div>
      <FolderTreePreview
        previewPath={previewPath}
        paths={paths}
        owner={owner}
        repo={repo}
        getHast={getHast}
      />
    </div>
  );
}
