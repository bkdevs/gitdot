import {
  getFolderEntries,
  parseRepositoryTree,
} from "@/(main)/[owner]/[repo]/util";
import { getRepositoryTree, NotFound } from "@/lib/dal";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { parseLineSelection } from "./util";

// TODO: think about how and whether we should do generateStaticParams
// i think? this causes prefetch behavior to fail as next.js isn't aware that the layouts are shared
// causing the initial html with file previews, commits, and the trees to be prefetched.
export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ owner: string; repo: string; filePath: string[] }>;
  searchParams: Promise<{
    lines?: string | string[];
    ref?: string;
  }>;
}) {
  const { owner, repo, filePath } = await params;

  const tree = await getRepositoryTree(owner, repo);
  if (!tree || tree === NotFound) return null;

  const filePathString = decodeURIComponent(filePath.join("/"));
  const { entries, folders } = parseRepositoryTree(tree);

  if (!entries.has(filePathString)) {
    return <div>File not found.</div>;
  } else if (folders.has(filePathString)) {
    return (
      <FolderViewer
        owner={owner}
        repo={repo}
        folderEntries={getFolderEntries(filePathString, folders, entries)}
      />
    );
  } else {
    const { lines, ref } = await searchParams;
    return (
      <FileViewer
        owner={owner}
        repo={repo}
        filePath={filePathString}
        selectedLines={parseLineSelection(lines)}
        selectedCommit={ref}
      />
    );
  }
}
