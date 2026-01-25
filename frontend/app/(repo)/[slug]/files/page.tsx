import { getRepositoryTree } from "@/lib/dal";
import { FolderViewer } from "../[...filePath]/ui/folder-viewer";
import { getFolderEntries, parseRepositoryTree } from "../util";

export default async function FilesPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug: repo } = await params;

  const tree = await getRepositoryTree("bkdevs", repo);
  if (!tree) return null;

  const { folders, entries } = parseRepositoryTree(tree);
  const rootEntries = getFolderEntries("", folders, entries);

  // Sort entries: folders first, then files
  const sortedEntries = rootEntries.sort((a, b) => {
    if (a.entry_type === b.entry_type) {
      return a.path.localeCompare(b.path);
    }
    return a.entry_type === "tree" ? -1 : 1;
  });

  return (
    <FolderViewer repo={repo} folderPath="" folderEntries={sortedEntries} />
  );
}
