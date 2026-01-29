import { getFolderEntries, parseRepositoryTree } from "@/[owner]/[repo]/util";
import { getRepositoryTree } from "@/lib/dal";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { parseLineSelection } from "./util";

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
  if (!tree) return null;

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
