import { getRepositoryCommits, getRepositoryTree } from "@/lib/dal";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoHeader } from "./ui/repo-header";
import { RepoSidebar } from "./ui/repo-sidebar";
import { parseRepositoryTree, renderFilePreviews } from "./util";

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ slug: string }>;
}>) {
  const { slug: repo } = await params;

  const [tree, commitsData] = await Promise.all([
    getRepositoryTree("bkdevs", repo),
    getRepositoryCommits("bkdevs", repo),
  ]);

  if (!tree) return null;

  const { folders, entries } = parseRepositoryTree(tree);
  const files = Array.from(entries.values()).filter(
    (entry) => entry.entry_type === "blob",
  );
  const commits = commitsData?.commits ?? [];

  // this still seems to incur _some_ latency, roughly 2-300 ms? setting up this promise stream does incur some blocking operation on the server,
  // even though there is no await being done and just awaited on in the client-side
  // TODO: test refactoring this to be a plain old client-side ajax request, may work better.
  const filePreviewsPromise = renderFilePreviews(files);

  return (
    <>
      <div className="flex flex-col h-screen w-full max-w-screen overflow-hidden">
        <RepoHeader repo={repo} />
        <div className="flex flex-1 min-h-0">
          <RepoSidebar
            repo={repo}
            folders={folders}
            entries={entries}
            commits={commits}
          />
          <main className="flex-1 min-w-0 overflow-auto">{children}</main>
        </div>
      </div>
      <RepoDialogs
        repo={repo}
        files={files}
        filePreviewsPromise={filePreviewsPromise}
      />
    </>
  );
}
