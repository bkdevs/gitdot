import { getAllDiffDataAction } from "@/actions/repository";
import { CommitPageClient } from "./ui/commit-page-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string; sha: string }>;
}) {
  const { owner, repo, sha } = await params;
  const allDiffDataPromise = getAllDiffDataAction(owner, repo, sha);

  return <CommitPageClient sha={sha} allDiffDataPromise={allDiffDataPromise} />;
}
