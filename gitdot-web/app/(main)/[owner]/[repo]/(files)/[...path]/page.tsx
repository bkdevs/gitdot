import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import type { Root } from "hast";
import { Suspense } from "react";
import { fetchResources } from "@/provider/server";
import { Loading } from "@/ui/loading";
import { PageClient } from "./page.client";
import { parseLineSelection } from "./util";

export type Resources = {
  blob: RepositoryBlobResource | null;
  hast: Root | null;
  paths: RepositoryPathsResource | null;
  commits: RepositoryCommitResource[] | null;
};

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; path: string[] }>;
  searchParams: Promise<{
    lines?: string | string[];
  }>;
}) {
  const { owner, repo, path } = await params;
  const { lines } = await searchParams;

  const filePathString = decodeURIComponent(path.join("/"));
  const selectedLines = parseLineSelection(lines);

  const { requests, promises } = fetchResources(owner, repo, {
    blob: (p) => p.getBlob(filePathString),
    hast: (p) => p.getHast(filePathString),
    paths: (p) => p.getPaths(),
    commits: (p) => p.getCommits(),
  });

  return (
    <Suspense fallback={<Loading />}>
      <PageClient
        owner={owner}
        repo={repo}
        selectedLines={selectedLines}
        filePath={filePathString}
        requests={requests}
        promises={promises}
      />
    </Suspense>
  );
}
