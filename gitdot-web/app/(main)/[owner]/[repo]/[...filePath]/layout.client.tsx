"use client";

import { File, Folder, FolderOpen, Undo2 } from "lucide-react";
import { useParams } from "next/navigation";
import { Fragment, Suspense, use, useEffect, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { getFolderEntries } from "../util";
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

  const [expanded, setExpanded] = useState<Set<string>>(() => {
    const s = new Set<string>();
    const parts = currentPath.split("/");
    for (let i = 1; i <= parts.length; i++) {
      s.add(parts.slice(0, i).join("/"));
    }
    return s;
  });

  useEffect(() => {
    setExpanded((prev) => {
      const next = new Set(prev);
      const parts = currentPath.split("/");
      for (let i = 1; i <= parts.length; i++) {
        next.add(parts.slice(0, i).join("/"));
      }
      return next;
    });
  }, [currentPath]);

  if (!paths) return null;

  const toggle = (path: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(path)) next.delete(path);
      else next.add(path);
      return next;
    });
  };

  const renderEntries = (
    parentPath: string,
    depth: number,
  ): React.ReactNode => {
    return getFolderEntries(parentPath, paths).map((entry) => {
      const name = entry.path.split("/").pop()!;
      const isFolder = entry.path_type === "tree";
      const isExpanded = expanded.has(entry.path);
      const isActive = currentPath === entry.path;

      return (
        <Fragment key={entry.path}>
          {isFolder ? (
            <div
              role="button"
              tabIndex={0}
              onClick={() => toggle(entry.path)}
              onKeyDown={(e) => (e.key === "Enter" || e.key === " ") && toggle(entry.path)}
              style={{ paddingLeft: `${0.5 + depth * 1}rem` }}
              className={`flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2 ${isActive ? "bg-sidebar" : ""}`}
              data-sidebar-item=""
              data-sidebar-item-active={isActive ? "true" : undefined}
            >
              {isExpanded ? (
                <FolderOpen className="size-4 shrink-0" />
              ) : (
                <Folder className="size-4 shrink-0" />
              )}
              <Link
                href={`/${owner}/${repo}/${entry.path}`}
                onClick={(e) => e.stopPropagation()}
                className="ml-2 truncate cursor-pointer hover:underline"
                prefetch={true}
              >
                {name}
              </Link>
            </div>
          ) : (
            <Link
              href={`/${owner}/${repo}/${entry.path}`}
              style={{ paddingLeft: `${0.5 + depth * 1}rem` }}
              className={`flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2 ${currentPath === entry.path ? "bg-sidebar" : ""}`}
              prefetch={true}
              data-sidebar-item=""
              data-sidebar-item-active={
                currentPath === entry.path ? "true" : undefined
              }
            >
              <File className="size-4 shrink-0" />
              <span className="ml-2 truncate">{name}</span>
            </Link>
          )}
          {isFolder && isExpanded && renderEntries(entry.path, depth + 1)}
        </Fragment>
      );
    });
  };

  return (
    <div className="flex flex-col w-full">
      <Link
        href={`/${owner}/${repo}`}
        className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
        prefetch={true}
      >
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Files
        </h3>
        <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
      </Link>
      {renderEntries("", 0)}
    </div>
  );
}
