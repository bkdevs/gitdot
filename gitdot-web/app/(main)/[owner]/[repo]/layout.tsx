import {
  getRepositoryCommits,
  getRepositoryPreview,
  getRepositoryTree,
  isRepositoryAdmin,
} from "@/dal";
import { RepoProvider } from "./context";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoSidebar } from "./ui/repo-sidebar";
import { renderFilePreviews } from "./util";

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;

  const tree = getRepositoryTree(owner, repo);
  const [commits, isAdmin] = await Promise.all([
    getRepositoryCommits(owner, repo),
    isRepositoryAdmin(owner, repo),
  ]);

  if (!commits) {
    return <div className="p-2 text-sm">Repository {repo} not found</div>;
  }

  // note: setting up this promise still seems to incur some blocking latency (200ms?)
  // TODO: experiment with just moving this to plain old ajax
  // note: this is because of renderFilePrevies and that blocking the next.js main thread
  const previewsPromise = (async () => {
    const data = await getRepositoryPreview(owner, repo);
    const entries = data && "entries" in data ? data.entries : [];
    return renderFilePreviews(entries);
  })();

  return (
    <RepoProvider tree={tree} commits={Promise.resolve(commits)}>
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full w-full">
        <RepoSidebar
          owner={owner}
          repo={repo}
          commits={commits?.commits ?? []}
          showSettings={isAdmin}
        />
        <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
          {children}
        </div>
      </div>
      <RepoDialogs
        owner={owner}
        repo={repo}
        previewsPromise={previewsPromise}
      />
    </RepoProvider>
  );
}
