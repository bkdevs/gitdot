"use client";

import { File, Folder, FolderOpen } from "lucide-react";
import { useParams } from "next/navigation";
import { Fragment, Suspense, use, useMemo } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import type { RepositoryPathsResource } from "gitdot-api";
import { getFolderEntries, getParentPath } from "../util";
import type { Resources } from "./layout";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

export function LayoutClient({
  owner,
  repo,
  requests,
  promises,
  children,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
  children: React.ReactNode;
}) {
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <>
      <Sidebar>
        <SidebarContent className="overflow-auto">
          <Suspense>
            <FileSidebarContent
              owner={owner}
              repo={repo}
              promises={resolvedPromises}
            />
          </Suspense>
        </SidebarContent>
      </Sidebar>
      <Suspense>
        <OverlayScroll>{children}</OverlayScroll>
      </Suspense>
    </>
  );
}

function FileSidebarContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const { filePath: filePathSegments } = useParams<{
    owner: string;
    repo: string;
    filePath: string[];
  }>();

  const currentPath = filePathSegments.join("/");
  const paths = use(promises.paths);
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
        const isFolder = file.path_type === "tree";
        const isActive = currentPath === fullPath;

        return (
          <Fragment key={file.path}>
            <FileRow
              filePath={filePath}
              href={`/${owner}/${repo}/${parentPath}/${filePath}`}
              isFolder={isFolder}
              isActive={isActive}
            />
            {isActive && isFolder && (
              <FolderChildren
                owner={owner}
                repo={repo}
                folderPath={fullPath}
                paths={paths}
              />
            )}
          </Fragment>
        );
      })}
    </div>
  );
}

function FolderChildren({
  owner,
  repo,
  folderPath,
  paths,
}: {
  owner: string;
  repo: string;
  folderPath: string;
  paths: RepositoryPathsResource;
}) {
  const children = getFolderEntries(folderPath, paths);
  return (
    <>
      {children.map((child) => {
        const childName = child.path.split("/").pop();
        if (!childName) return null;
        return (
          <FileRow
            key={child.path}
            filePath={childName}
            href={`/${owner}/${repo}/${folderPath}/${childName}`}
            isFolder={child.path_type === "tree"}
            isActive={false}
            indent={true}
          />
        );
      })}
    </>
  );
}

function FileRow({
  filePath,
  href,
  isFolder,
  isActive,
  indent = false,
}: {
  filePath: string;
  href: string;
  isFolder: boolean;
  isActive: boolean;
  indent?: boolean;
}) {
  const navigable = filePath !== "..";
  return (
    <Link
      href={href}
      className={`flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 ${indent ? "pl-6 pr-2" : "px-2"} ${
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
