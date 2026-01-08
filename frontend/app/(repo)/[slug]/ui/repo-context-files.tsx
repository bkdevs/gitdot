"use client";

import { File, Folder, FolderOpen } from "lucide-react";
import Link from "next/link";
import { useMemo } from "react";
import {
  type FolderFile,
  getFolderFiles,
  getParentPath,
} from "../[...filePath]/util";

function FileTreeItem({
  repo,
  file,
  contextPath,
  currentPath,
}: {
  repo: string;
  file: FolderFile;
  contextPath: string;
  currentPath: string;
}) {
  const isFolder = file.type === "folder";
  const fullPath =
    file.path === ".." || file.path === "."
      ? contextPath
      : contextPath
        ? `${contextPath}/${file.path}`
        : file.path;
  const isActive = currentPath === fullPath;

  return (
    <Link
      href={`/${repo}/${fullPath}`}
      className={`flex flex-row w-full px-2 h-9 items-center border-b select-none cursor-default text-sm font-mono ${
        isActive
          ? "bg-sidebar-primary text-sidebar-primary-foreground"
          : "hover:bg-accent/50"
      }`}
      prefetch={true}
    >
      {isFolder ? (
        isActive ? (
          <FolderOpen className="size-4" />
        ) : (
          <Folder className="size-4" />
        )
      ) : (
        <File className="size-4" />
      )}
      <span className="ml-2">{file.path}</span>
    </Link>
  );
}

export function RepoContextFiles({
  repo,
  folders,
  currentPath,
}: {
  repo: string;
  folders: Map<string, string[]>;
  currentPath: string;
}) {
  const contextFiles = useMemo(() => {
    const parentPath = getParentPath(currentPath);

    const files = getFolderFiles(parentPath, folders);
    return files.sort((a, b) => {
      if (a.type === b.type) {
        return a.path.localeCompare(b.path);
      }
      return a.type === "folder" ? -1 : 1;
    });
  }, [folders, currentPath]);

  const parentPath = getParentPath(currentPath);

  if (!folders) return null;

  return (
    <div className="flex flex-col w-full">
      {parentPath && (
        <FileTreeItem
          key=".."
          repo={repo}
          file={{ path: "..", type: "folder" }}
          contextPath={parentPath}
          currentPath={currentPath}
        />
      )}
      {contextFiles.map((file) => (
        <FileTreeItem
          key={file.path}
          repo={repo}
          file={file}
          contextPath={parentPath}
          currentPath={currentPath}
        />
      ))}
    </div>
  );
}
