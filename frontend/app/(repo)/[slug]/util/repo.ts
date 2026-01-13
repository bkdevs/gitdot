import type {
  RepositoryTree,
  RepositoryTreeEntry
} from "@/lib/dto";

export function parseRepositoryTree(tree: RepositoryTree): {
  entries: Map<string, RepositoryTreeEntry>;
  folders: Map<string, string[]>;
} {
  const entries = new Map<string, RepositoryTreeEntry>();
  const folders = new Map<string, string[]>();

  for (const entry of tree.entries) {
    entries.set(entry.path, entry);

    const segments = entry.path.split("/");
    const fileName = segments[segments.length - 1];

    if (segments.length === 1) {
      if (!folders.has("")) {
        folders.set("", []);
      }
      folders.get("")?.push(fileName);
    } else if (segments.length > 1) {
      const folder = segments.slice(0, -1).join("/");
      if (!folders.has(folder)) {
        folders.set(folder, []);
      }
      folders.get(folder)?.push(fileName);
    }
  }

  for (const arr of folders.values()) {
    arr.sort();
  }
  return { entries, folders };
}

export function getParentPath(currentPath: string): string {
  if (!currentPath) {
    return "";
  }

  const segments = currentPath.split("/");
  if (segments.length === 1) {
    return "";
  } else {
    return segments.slice(0, -1).join("/");
  }
}

export type FolderFile = {
  path: string;
  type: "file" | "folder";
};

export const getFolderEntries = (
  folderPath: string,
  folders: Map<string, string[]>,
  entries: Map<string, RepositoryTreeEntry>,
): RepositoryTreeEntry[] => {
  const files = folders.get(folderPath);
  if (!files) return [];
  return files
    .map((fileName) =>
      entries.get(folderPath ? `${folderPath}/${fileName}` : fileName),
    )
    .filter((entry): entry is RepositoryTreeEntry => entry !== undefined);
};
