import type {
  RepositoryBlobsResource,
  RepositoryCommitResource,
  RepositoryPathsResource,
  RepositorySettingsResource,
} from "gitdot-api";
import { GITDOT_SERVER_URL } from "@/dal/util";
import {
  fetchResources,
  type ResourcePromisesType,
  type ResourceRequestsType,
} from "@/provider/server";
import { RepoClient } from "./context";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoShortcuts } from "./ui/shortcuts";

type Resources = {
  paths: RepositoryPathsResource | null;
  commits: RepositoryCommitResource[] | null;
  blobs: RepositoryBlobsResource | null;
  settings: RepositorySettingsResource | null;
};
export type ResourcePromises = ResourcePromisesType<Resources>;
export type ResourceRequests = ResourceRequestsType<Resources>;

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;
  const { requests, promises } = fetchResources(owner, repo, {
    paths: (p) => p.getPaths(),
    commits: (p) => p.getCommits(),
    blobs: (p) => p.getBlobs(),
    settings: (p) => p.getSettings(),
  });

  return (
    <RepoClient
      owner={owner}
      repo={repo}
      serverUrl={GITDOT_SERVER_URL}
      serverRequests={requests}
      serverPromises={promises}
    >
      <RepoShortcuts />
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full">{children}</div>

      <RepoDialogs owner={owner} repo={repo} />
    </RepoClient>
  );
}
