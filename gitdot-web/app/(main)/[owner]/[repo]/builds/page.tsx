import type { RepositoryCommitResource } from "gitdot-api";
import { getBuilds, getRepositoryCommit } from "@/dal";
import { BuildsClient } from "./ui/builds-client";

export default async function Page({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;
  const builds = await getBuilds(owner, repo);
  if (!builds) return null;

  const commits = (await Promise.all(
    builds.map((build) => getRepositoryCommit(owner, repo, build.commit_sha)),
  )) as RepositoryCommitResource[];
  if (commits.some((c) => c === null)) return null;

  return (
    <BuildsClient owner={owner} repo={repo} builds={builds} commits={commits} />
  );
}
