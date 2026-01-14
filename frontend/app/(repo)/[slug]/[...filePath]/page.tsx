import { getFolderEntries, parseRepositoryTree } from "@/(repo)/[slug]/util";
import { getRepositoryTree } from "@/lib/dal";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import { parseLineSelection } from "./util";

export default async function Page({
  params,
  searchParams,
}: {
  params: Promise<{ slug: string; filePath: string[] }>;
  searchParams: Promise<{
    lines?: string | string[];
    ref?: string;
  }>;
}) {
  const { slug: repo, filePath } = await params;
  const tree = await getRepositoryTree("bkdevs", repo);
  if (!tree) return null;

  const filePathString = filePath.join("/");
  const { entries, folders } = parseRepositoryTree(tree);

  if (!entries.has(filePathString)) {
    return <div>File not found.</div>;
  } else if (folders.has(filePathString)) {
    return (
      <FolderViewer
        repo={repo}
        folderPath={filePathString}
        folderEntries={getFolderEntries(filePathString, folders, entries)}
      />
    );
  } else {
    const { lines, ref } = await searchParams;
    return (
      <FileViewer
        repo={repo}
        filePath={filePathString}
        selectedLines={parseLineSelection(lines)}
        selectedCommit={ref}
      />
    );
  }
}
