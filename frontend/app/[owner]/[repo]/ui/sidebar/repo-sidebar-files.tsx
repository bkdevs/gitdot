import { File, Folder, FolderOpen } from "lucide-react";
import { useMemo } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import Link from "@/ui/link";
import { getFolderEntries, getParentPath } from "../../util";

function FileRow({
  filePath,
  href,
  isFolder,
  isActive,
}: {
  filePath: string;
  href: string;
  isFolder: boolean;
  isActive: boolean;
}) {
  return (
    <Link
      href={href}
      className={`flex flex-row w-full px-2 h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 ${
        isActive && "bg-sidebar"
      }`}
      prefetch={true}
    >
      {isFolder ? (
        isActive ? (
          <FolderOpen className="size-4 shrink-0" />
        ) : (
          <Folder className="size-4 shrink-0" />
        )
      ) : (
        <File className="size-4 shrink-0" />
      )}
      <span className="ml-2 truncate">{filePath}</span>
    </Link>
  );
}

export function RepoSidebarFiles({
  owner,
  repo,
  folders,
  entries,
  currentPath,
}: {
  owner: string;
  repo: string;
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
  currentPath: string;
}) {
  const contextFiles = useMemo(() => {
    const parentPath = getParentPath(currentPath);

    const files = getFolderEntries(parentPath, folders, entries);
    return files.sort((a, b) => {
      if (a.entry_type === b.entry_type) {
        return a.path.localeCompare(b.path);
      }
      return a.entry_type === "tree" ? -1 : 1;
    });
  }, [folders, entries, currentPath]);
  const parentPath = getParentPath(currentPath);

  if (!folders) return null;

  return (
    <div className="flex flex-col w-full">
      <FileRow
        key=".."
        filePath={".."}
        href={
          parentPath
            ? `/${owner}/${repo}/${parentPath}`
            : `/${owner}/${repo}/files`
        }
        isFolder={true}
        isActive={false}
      />
      {contextFiles.map((file) => {
        const filePath = file.path.split("/").pop();
        if (!filePath) return null;
        const fullPath = parentPath ? `${parentPath}/${filePath}` : filePath; // account for root files

        return (
          <FileRow
            key={file.path}
            filePath={filePath}
            href={`/${owner}/${repo}/${parentPath}/${filePath}`}
            isFolder={file.entry_type === "tree"}
            isActive={currentPath === fullPath}
          />
        );
      })}
    </div>
  );
}
