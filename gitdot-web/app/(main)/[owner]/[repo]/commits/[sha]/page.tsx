import type { RepositoryCommitResource } from "gitdot-api";
import { renderCommitDiffAction } from "@/actions";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  commit: RepositoryCommitResource | null;
};

// TODO: this page is still somewhat laggy? not instant.
// consider ajax, but trade-off is preload...
export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    commit: (p) => p.getCommit(sha),
  });
  const diffEntriesPromise = renderCommitDiffAction(owner, repo, sha);

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
      diffEntriesPromise={diffEntriesPromise}
    />
  );
}
