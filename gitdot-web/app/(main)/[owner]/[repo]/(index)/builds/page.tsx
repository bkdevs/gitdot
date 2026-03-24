import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  builds: BuildResource[] | null;
  commits: RepositoryCommitResource[] | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    builds: (p) => p.getBuilds(),
    commits: (p) => p.getCommits(),
  });

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
    />
  );
}
