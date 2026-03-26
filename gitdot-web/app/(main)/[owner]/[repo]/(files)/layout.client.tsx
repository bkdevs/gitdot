"use client";

import type {
  RepositoryPathResource,
  RepositoryPathsResource,
} from "gitdot-api";
import { useParams } from "next/navigation";
import {
  Fragment,
  Suspense,
  use,
  useCallback,
  useLayoutEffect,
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
  const [rootPath, setRootPath] = useState("");

  if (!paths) return null;

  return (
    <>
      <FileTreeHeader owner={owner} repo={repo} rootPath={rootPath} />
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

const HEADER_CHAR_LIMIT = 32;

function FileTreeHeader({
  owner,
  repo,
  rootPath,
}: {
  owner: string;
  repo: string;
  rootPath: string;
}) {
  const allSegments = [repo, ...rootPath.split("/").filter(Boolean)];

  // Find how many segments to show from the right so the path fits
  let startIdx = 0;
  const fullDisplay = allSegments.join("/");
  if (fullDisplay.length > HEADER_CHAR_LIMIT) {
    for (let i = 1; i < allSegments.length; i++) {
      const display = `../${allSegments.slice(i).join("/")}`;
      if (display.length <= HEADER_CHAR_LIMIT || i === allSegments.length - 1) {
        startIdx = i;
        break;
      }
    }
  }

  const visibleSegments = allSegments.slice(startIdx);
  const truncated = startIdx > 0;

  return (
    <div className="sticky top-0 bg-background flex items-center border-b px-2 h-9 z-10 text-sm font-mono font-bold select-none overflow-hidden">
      <div className="flex items-center min-w-0 overflow-hidden whitespace-nowrap">
        {truncated && <span className="shrink-0">../</span>}
        {visibleSegments.map((segment, i) => {
          const globalIdx = startIdx + i;
          const segPath = allSegments.slice(1, globalIdx + 1).join("/");
          const isLast = i === visibleSegments.length - 1;
          return (
            <Fragment key={globalIdx}>
              <Link
                href={
                  segPath
                    ? `/${owner}/${repo}/${segPath}`
                    : `/${owner}/${repo}/files`
                }
                className={cn(
                  "cursor-pointer underline decoration-transparent hover:decoration-current",
                  isLast ? "truncate" : "shrink-0",
                )}
              >
                {segment}
              </Link>
              {!isLast && <span className="shrink-0">/</span>}
            </Fragment>
          );
        })}
      </div>
    </div>
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
    if (!isFolder) return;

    const isWithinCurrentRoot =
      rootPath === "" ||
      filePath === rootPath ||
      filePath.startsWith(`${rootPath}/`);

    if (isWithinCurrentRoot) {
      const segments = filePath.split("/");
      const rootDepth = rootPath === "" ? 0 : rootPath.split("/").length;
      const ancestors = Array.from(
        { length: segments.length - rootDepth - 1 },
        (_, i) => segments.slice(0, rootDepth + 1 + i).join("/"),
      );
      if (!ancestors.every((a) => expandedFolders.has(a))) {
        expandFolders(ancestors);
      }
    } else {
      updateRootPath(filePath);
    }
  }, [
    filePath,
    paths,
    rootPath,
    updateRootPath,
    expandFolders,
    expandedFolders,
  ]);

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

  return <>{renderRows(rootPath, 0)}</>;
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
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive && "bg-sidebar",
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
      </Link>
      <span className={cn("pl-px", expanded ? "opacity-100" : "opacity-40")}>
        /
      </span>
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
      style={{ paddingLeft: `${8 + depth * 16}px` }}
      className={cn(
        "flex flex-row w-full h-9 items-center border-b select-none cursor-default text-sm font-mono hover:bg-accent/50 pr-2",
        isActive && "bg-sidebar",
      )}
      data-sidebar-item=""
      data-sidebar-item-active={isActive}
    >
      <span className="truncate">{name}</span>
    </Link>
  );
}
