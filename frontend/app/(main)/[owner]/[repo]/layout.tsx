import {
  getRepositoryCommits,
  getRepositoryPreview,
  getRepositoryTree,
  NotFound,
} from "@/lib/dal";
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

  const [tree, commits] = await Promise.all([
    getRepositoryTree(owner, repo),
    getRepositoryCommits(owner, repo),
  ]);

  if (!tree || !commits) {
    return null;
  } else if (tree === NotFound || commits === NotFound) {
    return <div className="p-2 text-sm">Repository {repo} not found</div>;
  }

  const { folders, entries } = parseRepositoryTree(tree);
  const files = Array.from(entries.values()).filter(
    (entry) => entry.entry_type === "blob",
  );

  // note: setting up this promise still seems to incur some blocking latency (200ms?)
  // TODO: experiment with just moving this to plain old ajax
  const previewsPromise = (async () => {
    const data = await getRepositoryPreview(owner, repo);
    const entries =
      data && data !== NotFound && "entries" in data ? data.entries : [];
    return renderFilePreviews(entries);
  })();

  return (
    <>
      <div className="flex md:hidden h-full w-full p-2 text-sm">
        Mobile support to come.
      </div>

      <div className="hidden md:flex h-full w-full">
        <RepoSidebar
          owner={owner}
          repo={repo}
          folders={folders}
          entries={entries}
          commits={commits?.commits ?? []}
        />
        <div className="flex-1 min-w-0 overflow-auto scrollbar-thin">
          {children}
        </div>
      </div>
      <RepoDialogs
        owner={owner}
        repo={repo}
        files={files}
        previewsPromise={previewsPromise}
      />
    </>
  );
}
