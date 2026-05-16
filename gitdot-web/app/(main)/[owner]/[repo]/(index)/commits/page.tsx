import type {
  RepositoryCommitFilterResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { getRepository } from "@/dal/repository";
import { fetchResources } from "@/provider/server";
import { PageClient } from "./page.client";

export type Resources = {
  commits: RepositoryCommitResource[] | null;
  paths: RepositoryPathsResource | null;
  commitFilters: RepositoryCommitFilterResource[] | null;
};

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    commits: (p) => p.getCommits(),
    paths: (p) => p.getPaths(),
    commitFilters: (p) => p.getCommitFilters(),
  });
  const repository = await getRepository(owner, repo);

  return (
    <PageClient
      owner={owner}
      repo={repo}
      requests={requests}
      promises={promises}
      repository={repository}
    />
  );
}
