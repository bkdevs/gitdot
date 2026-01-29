import { getRepositoryTree } from "@/lib/dal";
import { FolderViewer } from "../[...filePath]/ui/folder-viewer";
import { getFolderEntries, parseRepositoryTree } from "../util";

export default async function FilesPage({
  params,
}: {
  params: Promise<{ owner: string; repo: string }>;
}) {
  const { owner, repo } = await params;

  const tree = await getRepositoryTree(owner, repo);
  if (!tree) return null;

  const { folders, entries } = parseRepositoryTree(tree);
  const rootEntries = getFolderEntries("", folders, entries);

  return <FolderViewer owner={owner} repo={repo} folderEntries={rootEntries} />;
}
