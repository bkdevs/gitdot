import { getRepositoryCommit } from "gitdot-client";
import { renderCommitDiff } from "gitdot-dal/server";
import { notFound } from "next/navigation";
import { PageClient } from "./page.client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const [commit, diffEntries] = await Promise.all([
    getRepositoryCommit(owner, repo, sha),
    renderCommitDiff(owner, repo, sha),
  ]);
  if (!commit) notFound();

  return (
    <PageClient
      owner={owner}
      repo={repo}
      commit={commit}
      diffEntries={diffEntries}
    />
  );
}
