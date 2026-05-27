"use client";

import type {
  RepositoryPathResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { Undo2 } from "lucide-react";
import { useParams } from "next/navigation";
import {
  Fragment,
  Suspense,
  use,
  useCallback,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import {
  type ResourcePromisesType,
  type ResourceResultType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { Loading } from "@/ui/loading";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { cn } from "@/util";
import type { Resources } from "./layout";

type ResourcePromises = ResourcePromisesType<Resources>;

export function LayoutClient({
  owner,
  repo,
  resources,
  children,
}: {
  owner: string;
  repo: string;
  resources: ResourceResultType<Resources>;
  children: React.ReactNode;
}) {
  const { path } = useParams<{ path: string[] }>();
  const filePath = path.join("/") ?? "";
  const resolvedPromises = useResolvePromises(owner, repo, resources);

  return (
    <>
      <Sidebar>
        <SidebarContent className="overflow-auto flex flex-col w-full">
          <div className="flex flex-col w-full">
            <FileSidebarHeader owner={owner} repo={repo} />
            <Suspense fallback={<Loading />}>
              <FileTree
                owner={owner}
                repo={repo}
                filePath={filePath}
                promises={resolvedPromises}
              />
            </Suspense>
          </div>
        </SidebarContent>
      </Sidebar>
      <Suspense>
        <OverlayScroll>{children}</OverlayScroll>
      </Suspense>
    </>
  );
}

function FileSidebarHeader({ owner, repo }: { owner: string; repo: string }) {
  return (
    <Link
      href={`/${owner}/${repo}/files`}
      className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default"
    >
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        Files
      </h3>
      <Undo2 size={14} className="text-muted-foreground -translate-y-px" />
    </Link>
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
  const paths = use(promises.paths);

  if (!paths) return null;

  return (
    <FileTreeRows owner={owner} repo={repo} filePath={filePath} paths={paths} />
  );
}

function FileTreeRows({
  owner,
  repo,
  filePath,
  paths,
}: {
  owner: string;
  repo: string;
  filePath: string;
  paths: RepositoryPathsResource;
}) {
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(
    new Set(),
  );
  const expandedFoldersRef = useRef(expandedFolders);
  expandedFoldersRef.current = expandedFolders;

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

  const expandFolders = useCallback((paths: string[]) => {
    setExpandedFolders((prev) => new Set([...prev, ...paths]));
  }, []);

  useLayoutEffect(() => {
    if (filePath === "") return;

    const isFolder = paths.entries.some(
      (e) => e.path === filePath && e.path_type === "tree",
    );
    const segments = filePath.split("/");
    const targetSegments = isFolder ? segments : segments.slice(0, -1);
    const ancestors = targetSegments.map((_, i) =>
      targetSegments.slice(0, i + 1).join("/"),
    );
    if (
      ancestors.length > 0 &&
      !ancestors.every((a) => expandedFoldersRef.current.has(a))
    ) {
      expandFolders(ancestors);
    }
  }, [filePath, paths, expandFolders]);

  const renderRows = (parentPath: string, depth: number): React.ReactNode => {
    const entries = getFolderEntries(parentPath, paths);
    return entries.map((entry, index) => {
      const isFolder = entry.path_type === "tree";
      const isExpanded = expandedFolders.has(entry.path);
      const isActive = filePath === entry.path;
      const isFirst = depth === 0 && index === 0;

      return (
        <Fragment key={entry.path}>
          {isFolder ? (
            <FolderRow
              owner={owner}
              repo={repo}
              entry={entry}
              depth={depth}
              isActive={isActive}
              isFirst={isFirst}
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
              isFirst={isFirst}
            />
          )}
          {isFolder && isExpanded && renderRows(entry.path, depth + 1)}
        </Fragment>
      );
    });
  };

  return <>{renderRows("", 0)}</>;
}

function RowGutter({ depth }: { depth: number }) {
  if (depth === 0) return null;
  return (
    <>
      {Array.from({ length: depth }, (_, i) => {
        const left = 10 + i * 16;
        return (
          <span
            key={`gutter-${left}px`}
            className="absolute top-0 bottom-0 w-px bg-border"
            style={{ left: `${left}px` }}
          />
        );
      })}
    </>
  );
}

function FolderRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
  isFirst,
  expanded,
  setExpanded,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
  isFirst: boolean;
  expanded: boolean;
  setExpanded: () => void;
}) {
  const name = entry.path.split("/").pop();

  return (
    <button
      type="button"
      onClick={() => setExpanded()}
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "relative flex flex-row w-full h-8 items-center select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive &&
          cn(
            "bg-sidebar border-b border-b-border",
            !isFirst && "border-t border-t-border",
          ),
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive ? "true" : undefined}
    >
      <RowGutter depth={depth} />
      <Link
        href={`/${owner}/${repo}/${entry.path}`}
        onClick={(e) => e.stopPropagation()}
        className="inline-flex items-center truncate cursor-pointer"
      >
        <span className="underline decoration-transparent hover:decoration-current">
          {name}
        </span>
        {expanded && "/"}
      </Link>
    </button>
  );
}

function FileRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
  isFirst,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
  isFirst: boolean;
}) {
  const name = entry.path.split("/").pop();

  return (
    <Link
      href={`/${owner}/${repo}/${entry.path}`}
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "relative flex flex-row w-full h-8 items-center select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive &&
          cn(
            "bg-sidebar border-b border-b-border",
            !isFirst && "border-t border-t-border",
          ),
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive}
    >
      <RowGutter depth={depth} />
      <span className="truncate">{name}</span>
    </Link>
  );
}
