import { fetchResources } from "@/provider/server";
import type { RepositoryBlobResource } from "gitdot-api";
import { PageClient } from "./page.client";

export type Resources = {
  readme: RepositoryBlobResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    readme: (p) => p.getBlob("README.md"),
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
