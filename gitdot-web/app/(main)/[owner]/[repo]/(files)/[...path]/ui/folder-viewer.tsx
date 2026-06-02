"use client";

import type { RepositoryPathsResource } from "gitdot-api";
import { ClientProvider } from "gitdot-dal/client";
import type { Root } from "hast";
import { useParams } from "next/navigation";
import { useRepoSynced } from "@/(main)/[owner]/[repo]/ui/use-repo-synced";
import { FolderPathPreview } from "./folder-path-preview";
import { FolderTree } from "./folder-tree";
import { FolderViewerProvider } from "./folder-viewer-context";

export function FolderViewer({
  path,
  paths,
}: {
  path: string;
  paths: RepositoryPathsResource | null;
}) {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const synced = useRepoSynced(owner, repo);

  const getHast = (p: string): Promise<Root | null> =>
    ClientProvider.instance.getHast(owner, repo, p);

  if (!paths) return null;

  return (
    <FolderViewerProvider pinFiles={true}>
      <div className="flex w-full h-full min-h-0 overflow-hidden">
        <div className="w-[45%] shrink-0 border-r h-full">
          <FolderTree path={path} owner={owner} repo={repo} paths={paths} />
        </div>
        <FolderPathPreview
          paths={paths}
          owner={owner}
          repo={repo}
          getHast={getHast}
          synced={synced}
        />
      </div>
    </FolderViewerProvider>
  );
}
