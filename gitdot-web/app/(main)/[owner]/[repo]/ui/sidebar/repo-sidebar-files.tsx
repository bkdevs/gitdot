import { File, Folder, FolderOpen } from "lucide-react";
import { use, useMemo } from "react";
import Link from "@/ui/link";
import { useRepoContext } from "../../context";
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
  currentPath,
}: {
  owner: string;
  repo: string;
  currentPath: string;
  }) {
  const paths = use(useRepoContext().paths);
  const parentPath = getParentPath(currentPath);

  const contextFiles = useMemo(
    () => getFolderEntries(parentPath, paths),
    [parentPath, paths],
  );

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
        const fullPath = parentPath ? `${parentPath}/${filePath}` : filePath;

        return (
          <FileRow
            key={file.path}
            filePath={filePath}
            href={`/${owner}/${repo}/${parentPath}/${filePath}`}
            isFolder={file.path_type === "tree"}
            isActive={currentPath === fullPath}
          />
        );
      })}
    </div>
  );
}
