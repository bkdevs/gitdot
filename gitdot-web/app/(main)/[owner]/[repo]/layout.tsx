import { getUserMetadata } from "@/lib/supabase";
import { ApiProvider } from "@/provider/api";
import { RepoClient } from "./context";
import { RepoResources } from "./resources";
import { RepoShortcuts } from "./shortcuts";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoScroll } from "./ui/repo-scroll";
import { RepoSidebar } from "./ui/repo-sidebar";

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;
  const { username, orgs } = await getUserMetadata();

  const serverPromises = new ApiProvider(owner, repo).fetch(RepoResources);
  const isAdmin = username === owner || orgs.includes(`${owner}:admin`);

  return (
    <RepoClient owner={owner} repo={repo} serverPromises={serverPromises}>
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
