import type { RepositoryCommitResource } from "gitdot-api";
import { renderCommitDiffAction } from "@/actions";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  commit: RepositoryCommitResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    commit: (p) => p.getCommit(sha),
  });
  const diffPromise = renderCommitDiffAction(owner, repo, sha);

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
      diffPromise={diffPromise}
    />
  );
}
