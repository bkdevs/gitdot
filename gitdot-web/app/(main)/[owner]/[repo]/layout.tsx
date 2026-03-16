import { getUserMetadata } from "@/lib/supabase";
import { ApiProvider } from "@/provider/api";
import { RepoClient } from "./context";
import { Resources } from "./resources";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoSidebar } from "./ui/repo-sidebar";

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;

  const provider = new ApiProvider(owner, repo);
  const serverPromises = provider.fetch(Resources);
  const { username, orgs } = await getUserMetadata();
  const isAdmin = username === owner || orgs.includes(`${owner}:admin`);

  return (
    <RepoClient owner={owner} repo={repo} serverPromises={serverPromises}>
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full w-full">
        <RepoSidebar owner={owner} repo={repo} showSettings={isAdmin} />
        <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
          {children}
        </div>
      </div>

      <RepoDialogs owner={owner} repo={repo} />
    </RepoClient>
  );
}
