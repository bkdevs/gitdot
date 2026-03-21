import { getUserMetadata } from "@/lib/supabase";
import {
  fetchResources,
  type ResourceDefinition,
  type ResourcePromisesType,
  type ResourceRequestsType,
} from "@/provider/server";
import { RepoClient } from "./context";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoScroll } from "./ui/repo-scroll";
import { RepoShortcuts } from "./ui/repo-shortcuts";
import { RepoSidebar } from "./ui/repo-sidebar";

const resources = {
  paths: (p) => p.getPaths(),
  commits: (p) => p.getCommits(),
  blobs: (p) => p.getBlobs(),
  settings: (p) => p.getSettings(),
} satisfies ResourceDefinition;

// works as server types can be imported by client-components but not server consts
export type ResourcePromises = ResourcePromisesType<typeof resources>;
export type ResourceRequests = ResourceRequestsType<typeof resources>;

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;
  const { username, orgs } = await getUserMetadata();
  const { requests, promises } = fetchResources(owner, repo, resources);
  const isAdmin = username === owner || orgs.includes(`${owner}:admin`);

  return (
    <RepoClient
      owner={owner}
      repo={repo}
      serverRequests={requests}
      serverPromises={promises}
    >
      <RepoShortcuts />
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full w-full">
        <RepoSidebar owner={owner} repo={repo} showSettings={isAdmin} />
        <RepoScroll>{children}</RepoScroll>
      </div>

      <RepoDialogs owner={owner} repo={repo} />
    </RepoClient>
  );
}
