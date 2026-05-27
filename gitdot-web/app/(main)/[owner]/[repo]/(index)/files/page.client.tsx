"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceResultType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import { FolderViewer } from "../../(files)/[...path]/ui/folder-viewer";
import type { Resources } from "./page";

type ResourcePromises = ResourcePromisesType<Resources>;

export function PageClient({
  owner,
  repo,
  resources,
}: {
  owner: string;
  repo: string;
  resources: ResourceResultType<Resources>;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, resources);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({ promises }: { promises: ResourcePromises }) {
  const paths = use(promises.paths);

  if (!paths) {
    return <div className="p-2 font-mono h-9 text-sm">no files found</div>;
  } else {
    return <FolderViewer path="" paths={paths} />;
  }
}
