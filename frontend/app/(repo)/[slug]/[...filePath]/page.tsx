import { getRepositoryTree } from "@/lib/dal";
import { FileViewer } from "./ui/file-viewer";
import { FolderViewer } from "./ui/folder-viewer";
import {
  getFolderFiles,
  parseLineSelection,
  parseRepositoryTree,
} from "./util";

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
  const { filePaths, folders } = parseRepositoryTree(tree);

  if (!folders.has(filePathString) && !filePaths.has(filePathString)) {
    return <div>File not found.</div>;
  } else if (folders.has(filePathString)) {
    return (
      <FolderViewer
        repo={repo}
        folderPath={filePathString}
        folderFiles={getFolderFiles(filePathString, folders)}
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
