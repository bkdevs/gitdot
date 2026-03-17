import { Suspense } from "react";
import { renderCommitDiffAction } from "@/actions";
import { CommitClient } from "./ui/commit-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const diffData = renderCommitDiffAction(owner, repo, sha);

  return (
    <Suspense>
      <CommitClient sha={sha} diffData={diffData} />
    </Suspense>
  );
}
