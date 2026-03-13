import type {
  RepositoryPathResource,
  RepositoryPathsResource,
} from "gitdot-api";

export function getFolderEntries(
  folderPath: string,
  paths: RepositoryPathsResource,
): RepositoryPathResource[] {
  if (folderPath !== "") {
    const folderExists = paths.entries.some(
      (e) => e.path === folderPath && e.path_type === "tree",
    );
    if (!folderExists) return [];
  }

  const entries = paths.entries.filter((e) => {
    if (folderPath === "") {
      return !e.path.includes("/");
    }
    const prefix = `${folderPath}/`;
    if (!e.path.startsWith(prefix)) return false;
    const remainder = e.path.slice(prefix.length);
    return !remainder.includes("/");
  });

  return entries.sort((a, b) => {
    if (a.path_type === b.path_type) return a.path.localeCompare(b.path);
    return a.path_type === "tree" ? -1 : 1;
  });
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
