import type { RepositoryPathsResource } from "gitdot-api";
import { getRepository } from "gitdot-client";
import { fetchResources } from "gitdot-dal/server";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoNotFound } from "./ui/repo-not-found";
import { RepoSync } from "./ui/repo-sync";
import { RepoTracker } from "./ui/repo-tracker";
import { RepoShortcuts } from "./ui/shortcuts";

export type Resources = {
  paths: RepositoryPathsResource | null;
};

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;
  const repository = await getRepository(owner, repo);
  if (!repository) return <RepoNotFound owner={owner} repo={repo} />;

  const resources = fetchResources({
    paths: (p) => p.getPaths(owner, repo),
  });

  return (
    <RepoSync owner={owner} repo={repo}>
      <RepoTracker owner={owner} repo={repo} />
      <RepoShortcuts />
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full">{children}</div>

      <RepoDialogs owner={owner} repo={repo} resources={resources} />
    </RepoSync>
  );
}
