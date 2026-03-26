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
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { getFolderEntries } from "@/(main)/[owner]/[repo]/util";
import Link from "@/ui/link";
import { Loading } from "@/ui/loading";
import { OverlayScroll } from "@/ui/scroll";
import { Sidebar, SidebarContent } from "@/ui/sidebar";
import { cn } from "@/util";
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
  const filePath = path.join("/") ?? "";
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);

  return (
    <>
      <Sidebar>
        <SidebarContent className="overflow-auto flex flex-col w-full">
          <div className="flex flex-col w-full">
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
  const [rootPath, setRootPath] = useState(() =>
    filePath.split("/").slice(0, -1).join("/"),
  );

  if (!paths) return null;

  return (
    <>
      <FileTreeHeader
        owner={owner}
        repo={repo}
        rootPath={rootPath}
        onNavigateUp={() =>
          setRootPath(rootPath.split("/").slice(0, -1).join("/"))
        }
      />
      <FileTreeRows
        owner={owner}
        repo={repo}
        filePath={filePath}
        paths={paths}
        rootPath={rootPath}
        setRootPath={setRootPath}
      />
    </>
  );
}


function FileTreeHeader({
  owner,
  repo,
  rootPath,
  onNavigateUp,
}: {
  owner: string;
  repo: string;
  rootPath: string;
  onNavigateUp: () => void;
}) {
  const isRoot = rootPath === "";

  return (
    <Link
      href={`/${owner}/${repo}/files`}
      onClick={
        !isRoot
          ? (e) => {
              e.preventDefault();
              onNavigateUp();
            }
          : undefined
      }
      className="sticky top-0 bg-background flex items-center justify-between border-b px-2 h-9 z-10 hover:bg-accent/50 cursor-default select-none"
    >
      {isRoot ? (
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Files
        </h3>
      ) : (
        <span className="font-mono text-sm truncate">
          <span className="text-base leading-none">•</span>/{rootPath}/
        </span>
      )}
      <Undo2 size={14} className="text-muted-foreground -translate-y-px shrink-0" />
    </Link>
  );
}

function FileTreeRows({
  owner,
  repo,
  filePath,
  paths,
  rootPath,
  setRootPath,
}: {
  owner: string;
  repo: string;
  filePath: string;
  paths: RepositoryPathsResource;
  rootPath: string;
  setRootPath: (path: string) => void;
}) {
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(
    new Set(),
  );
  const expandedFoldersRef = useRef(expandedFolders);
  expandedFoldersRef.current = expandedFolders;

  const updateRootPath = useCallback(
    (path: string) => {
      setRootPath(path);
      setExpandedFolders(new Set());
    },
    [setRootPath],
  );

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
    if (filePath === "") {
      updateRootPath("");
      return;
    }

    const isFolder = paths.entries.some(
      (e) => e.path === filePath && e.path_type === "tree",
    );

    // For files, rootPath and expand logic is based on the parent folder
    const targetPath = isFolder
      ? filePath
      : filePath.split("/").slice(0, -1).join("/");

    if (targetPath === rootPath) return;

    const isWithinCurrentRoot =
      rootPath === "" ||
      targetPath === rootPath ||
      targetPath.startsWith(`${rootPath}/`);

    if (isWithinCurrentRoot) {
      const segments = filePath.split("/");
      const rootDepth = rootPath === "" ? 0 : rootPath.split("/").length;
      const ancestors = Array.from(
        { length: segments.length - rootDepth - 1 },
        (_, i) => segments.slice(0, rootDepth + 1 + i).join("/"),
      );
      if (!ancestors.every((a) => expandedFoldersRef.current.has(a))) {
        expandFolders(ancestors);
      }
    } else {
      updateRootPath(targetPath);
    }
  }, [
    filePath,
    paths,
    rootPath,
    updateRootPath,
    expandFolders,
  ]);

  const renderRows = (parentPath: string, depth: number): React.ReactNode => {
    const entries = getFolderEntries(parentPath, paths);
    return entries.map((entry, index) => {
      const isFolder = entry.path_type === "tree";
      const isExpanded = expandedFolders.has(entry.path);
      const isActive = filePath === entry.path;
      const isLast = index === entries.length - 1;

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
              isLast={isLast}
              setExpanded={() => toggleFolder(entry.path)}
              count={getFolderEntries(entry.path, paths).length}
            />
          ) : (
            <FileRow
              owner={owner}
              repo={repo}
              entry={entry}
              depth={depth}
              isActive={isActive}
              isLast={isLast}
            />
          )}
          {isFolder && isExpanded && renderRows(entry.path, depth + 1)}
        </Fragment>
      );
    });
  };

  return <>{renderRows(rootPath, 0)}</>;
}

function RowGutter({ depth }: { depth: number }) {
  if (depth === 0) return null;
  return (
    <>
      {Array.from({ length: depth }, (_, i) => (
        <span
          key={i}
          className="absolute top-0 bottom-0 w-px bg-border"
          style={{ left: `${16 + i * 16}px` }}
        />
      ))}
    </>
  );
}

function FolderRow({
  owner,
  repo,
  depth,
  entry,
  isActive,
  expanded,
  isLast,
  setExpanded,
  count,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
  expanded: boolean;
  isLast: boolean;
  setExpanded: () => void;
  count: number;
}) {
  const name = entry.path.split("/").pop();

  return (
    <button
      type="button"
      onClick={() => setExpanded()}
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "relative flex flex-row w-full h-9 items-center select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        (depth === 0 || (isLast && depth === 1)) && "border-b",
        isActive && "bg-sidebar border-t border-b",
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
        <span className="underline decoration-transparent hover:decoration-current">{name}</span>
        {expanded && "/"}
        {!expanded && (
          <span className="ml-1 text-xs text-muted-foreground">({count})</span>
        )}
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
  isLast,
}: {
  owner: string;
  repo: string;
  depth: number;
  entry: RepositoryPathResource;
  isActive: boolean;
  isLast: boolean;
}) {
  const name = entry.path.split("/").pop();

  return (
    <Link
      href={`/${owner}/${repo}/${entry.path}`}
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "relative flex flex-row w-full h-9 items-center select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        (depth === 0 || (isLast && depth === 1)) && "border-b",
        isActive && "bg-sidebar border-t border-b",
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive}
    >
      <RowGutter depth={depth} />
      <span className="truncate">{name}</span>
    </Link>
  );
}
