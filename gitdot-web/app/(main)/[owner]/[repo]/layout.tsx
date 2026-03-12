import {
  getRepositoryCommits,
  getRepositoryPreview,
  getRepositoryTree,
} from "@/dal";
import { RepoProvider } from "./context";
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

  const tree = getRepositoryTree(owner, repo);
  const commits = getRepositoryCommits(owner, repo);
  const preview = getRepositoryPreview(owner, repo);
  // const isAdmin = await isRepositoryAdmin(owner, repo);

  return (
    <RepoProvider tree={tree} commits={commits} preview={preview}>
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full w-full">
        <RepoSidebar owner={owner} repo={repo} showSettings={true} />
        <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
          {children}
        </div>
      </div>

      <RepoDialogs owner={owner} repo={repo} />
    </RepoProvider>
  );
}
