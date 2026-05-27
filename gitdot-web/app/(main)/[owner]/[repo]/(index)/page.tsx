import type { RepositoryBlobResource } from "gitdot-api";
import { getRepository, getRepositoryActivity } from "@/dal/repository";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";
import { RepoPanel } from "./ui/repo-panel";

export type Resources = {
  readme: RepositoryBlobResource | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const repository = await getRepository(owner, repo);
  if (!repository) return null;

  const resources = fetchResources(owner, repo, {
    readme: (p) => p.getBlob("README.md"),
  });

  const activityPromise = getRepositoryActivity(owner, repo);

  return (
    <div className="flex h-full w-full overflow-hidden">
      <PageClient owner={owner} repo={repo} resources={resources} />
      <RepoPanel repository={repository} activityPromise={activityPromise} />
    </div>
  );
}
