"use client";

import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { cn } from "@/util";
import type { RepositoryPathResource } from "gitdot-api";
import { Undo2 } from "lucide-react";
import { useParams } from "next/navigation";
import { Fragment, Suspense, use, useEffect, useState } from "react";
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
  const { path } = useParams<{ path: string[] }>();
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);

  return (
    <>
      <Sidebar>
        <SidebarContent className="overflow-auto">
          <Suspense>
            <FileTree
              owner={owner}
              repo={repo}
              filePath={path.join("/") ?? ""}
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

function FileTree({
  owner,
  repo,
  filePath,
  promises,
}: {
  owner: string;
  repo: string;
  filePath: string;
  promises: ResourcePromises;
}) {
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(() => {
    const s = new Set<string>();
    const parts = filePath.split("/");
    for (let i = 1; i <= parts.length; i++) {
      s.add(parts.slice(0, i).join("/"));
    }
    return s;
  });

  useEffect(() => {
    setExpandedFolders((prev) => {
      const next = new Set(prev);
      const parts = filePath.split("/");
      for (let i = 1; i <= parts.length; i++) {
        next.add(parts.slice(0, i).join("/"));
      }
      return next;
    });
  }, [filePath]);

  const toggleFolder = (path: string) => {
    setExpandedFolders((prev) => {
      const next = new Set(prev);
      if (next.has(path)) {
        for (const p of next) {
          if (p === path || p.startsWith(`${path}/`)) {
            next.delete(p);
          }
        }
      } else {
        next.add(path);
      }

      return next;
    });
  };

  const paths = use(promises.paths);
  if (!paths) return null;

  const renderRows = (parentPath: string, depth: number): React.ReactNode => {
    return getFolderEntries(parentPath, paths).map((entry) => {
      const isFolder = entry.path_type === "tree";
      const isExpanded = expandedFolders.has(entry.path);
      const isActive = filePath === entry.path;

      return (
        <Fragment key={entry.path}>
          {isFolder ? (
            <FolderRow
              owner={owner}
              repo={repo}
              entry={entry}
              depth={depth}
              isActive={isActive}
              expanded={isExpanded}
              setExpanded={() => toggleFolder(entry.path)}
            />
          ) : (
            <FileRow
              owner={owner}
              repo={repo}
              entry={entry}
              depth={depth}
              isActive={isActive}
            />
          )}
          {isFolder && isExpanded && renderRows(entry.path, depth + 1)}
        </Fragment>
      );
    });
  };

  return (
    <div className="flex flex-col w-full">
      <Link
        href={`/${owner}/${repo}`}
        className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
      >
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Files
        </h3>
        <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
      </Link>
      {renderRows("", 0)}
    </div>
  );
}
function FolderRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
  expanded,
  setExpanded,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
  expanded: boolean;
  setExpanded: () => void;
}) {
  const name = entry.path.split("/").pop();

  return (
    <button
      type="button"
      onClick={() => setExpanded()}
      style={{ paddingLeft: `${8 + depth * 12}px` }}
      className={cn(
        "flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive && "bg-sidebar"
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive ? "true" : undefined}
    >
      <Link
        href={`/${owner}/${repo}/${entry.path}`}
        onClick={(e) => e.stopPropagation()}
        className="truncate cursor-pointer underline decoration-transparent hover:decoration-current transition-colors duration-300"
      >
        {name}
      </Link><span className={cn("pl-0.5", expanded ? "opacity-100" : "opacity-40")}>/</span>
    </button>
  );
}

function FileRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
}) {
  const name = entry.path.split("/").pop();

  return (
    <Link
      href={`/${owner}/${repo}/${entry.path}`}
      style={{ paddingLeft: `${8 + depth * 12}px` }}
      className={cn(
        "flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive && "bg-sidebar"
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive}
    >
      <span className="truncate">{name}</span>
    </Link>
  );
}
