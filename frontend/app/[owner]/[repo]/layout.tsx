import { getRepositoryCommits, getRepositoryTree } from "@/lib/dal";
import { RepoDialogs } from "./ui/dialog/repo-dialogs";
import { RepoSidebar } from "./ui/repo-sidebar";
import { parseRepositoryTree, renderFilePreviews } from "./util";

export default async function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: Promise<{ owner: string; repo: string }>;
}>) {
  const { owner, repo } = await params;

  const [tree, commitsData] = await Promise.all([
    getRepositoryTree(owner, repo),
    getRepositoryCommits(owner, repo),
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
      <div className="flex h-full w-full">
        <RepoSidebar
          owner={owner}
          repo={repo}
          folders={folders}
          entries={entries}
          commits={commits}
        />
        <div className="flex-1 min-w-0 overflow-auto">{children}</div>
      </div>
      <RepoDialogs
        owner={owner}
        repo={repo}
        files={files}
        filePreviewsPromise={filePreviewsPromise}
      />
    </>
  );
}
