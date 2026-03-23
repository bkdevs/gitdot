"use client";

import { File, Folder, FolderOpen } from "lucide-react";
import { useParams } from "next/navigation";
import { use, useMemo } from "react";
import Link from "@/ui/link";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { useRepoContext } from "../../context";
import { getFolderEntries, getParentPath } from "../../util";

export function FileSidebar() {
  return (
    <Sidebar>
      <SidebarContent className="overflow-auto">
        <FileSidebarContent />
      </SidebarContent>
    </Sidebar>
  );
}

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
  const navigable = filePath !== "..";
  return (
    <Link
      href={href}
      className={`flex flex-row w-full px-2 h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 ${
        isActive && "bg-sidebar"
      }`}
      prefetch={true}
      data-sidebar-item={navigable ? "" : undefined}
      data-sidebar-item-active={navigable && isActive ? "true" : undefined}
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

function FileSidebarContent() {
  const {
    owner,
    repo,
    filePath: filePathSegments,
  } = useParams<{
    owner: string;
    repo: string;
    filePath: string[];
  }>();

  const currentPath = filePathSegments.join("/");
  const paths = use(useRepoContext().paths);
  const parentPath = getParentPath(currentPath);
  const contextFiles = useMemo(
    () => (paths ? getFolderEntries(parentPath, paths) : []),
    [parentPath, paths],
  );
  if (!paths) return null;

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
