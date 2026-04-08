"use client";

import Link from "@/ui/link";
import { cn } from "@/util";
import type { FolderTreeRowData } from "../util";
import { useFolderViewerContext } from "./folder-viewer-context";

export type { FolderTreeRowData as TreeRowData } from "../util";

export function FolderTreeRow({
  row,
  owner,
  repo,
  onMouseEnter,
  onClick,
  onFileClick,
  absolutePaths,
}: {
  row: FolderTreeRowData;
  owner: string;
  repo: string;
  onMouseEnter: () => void;
  onClick: (path: string) => void;
  onFileClick: (path: string) => void;
  absolutePaths: boolean;
}) {
  return row.isTree ? (
    <TreeRowFolder
      row={row}
      owner={owner}
      repo={repo}
      onMouseEnter={onMouseEnter}
      onClick={onClick}
      absolutePaths={absolutePaths}
    />
  ) : (
    <TreeRowFile
      row={row}
      owner={owner}
      repo={repo}
      onMouseEnter={onMouseEnter}
      onFileClick={onFileClick}
      absolutePaths={absolutePaths}
    />
  );
}

function TreeRowFolder({
  row,
  owner,
  repo,
  onMouseEnter,
  onClick,
  absolutePaths,
}: {
  row: FolderTreeRowData;
  owner: string;
  repo: string;
  onMouseEnter: () => void;
  onClick: (path: string) => void;
  absolutePaths: boolean;
}) {
  const { hoveredPath } = useFolderViewerContext();
  const isActive = hoveredPath === row.path;

  return (
    <button
      type="button"
      className={cn(
        "flex items-stretch gap-1 font-mono text-sm h-6 shrink-0 select-none ring-0 outline-0 w-full pl-1 pr-2",
        isActive && "bg-accent/50",
      )}
      onMouseEnter={onMouseEnter}
      onClick={() => onClick(row.path)}
    >
      <TreeRowGutter depth={row.depth} isLast={row.isLast} />
      <Link
        href={`/${owner}/${repo}/${row.path}`}
        className={cn("inline-flex items-center cursor-pointer")}
        onClick={(e) => {
          e.stopPropagation();
        }}
      >
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        <span className="underline decoration-transparent hover:decoration-current transition-colors duration-300">
          {row.name}
        </span>
        {row.isExpanded && "/"}
        {!row.isExpanded && (
          <span className="ml-1 text-xs text-muted-foreground inline-flex items-center">
            ({row.fileCount})
          </span>
        )}
      </Link>
    </button>
  );
}

export function TreeRowFile({
  row,
  owner,
  repo,
  onMouseEnter,
  onFileClick,
  absolutePaths,
}: {
  row: FolderTreeRowData;
  owner: string;
  repo: string;
  onMouseEnter: () => void;
  onFileClick: (path: string) => void;
  absolutePaths: boolean;
}) {
  const { pinFiles, pinnedPath, hoveredPath } = useFolderViewerContext();
  const pinned = pinnedPath === row.path;
  const isActive = pinned || hoveredPath === row.path;

  const rowClass = cn(
    "flex items-stretch gap-1 font-mono text-sm h-6 shrink-0 select-none ring-0 outline-0 cursor-default px-1 w-full",
    isActive && "bg-accent/50",
  );

  if (!pinFiles) {
    return (
      <Link
        href={`/${owner}/${repo}/${row.path}`}
        data-path={row.path}
        className={rowClass}
        onMouseEnter={onMouseEnter}
      >
        <TreeRowGutter depth={row.depth} isLast={row.isLast} />
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        <span className="underline decoration-transparent hover:decoration-current transition-colors duration-300 flex items-center">
          {row.name}
        </span>
      </Link>
    );
  }

  return (
    <button
      type="button"
      data-path={row.path}
      className={rowClass}
      onMouseEnter={onMouseEnter}
      onClick={() => onFileClick(row.path)}
    >
      <TreeRowGutter depth={row.depth} isLast={row.isLast} />
      <Link
        href={`/${owner}/${repo}/${row.path}`}
        className="inline-flex items-center cursor-pointer"
        onClick={(e) => e.stopPropagation()}
      >
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        <span className="underline decoration-transparent hover:decoration-current transition-colors duration-300">
          {row.name}
        </span>
      </Link>
    </button>
  );
}

function TreeRowGutter({ depth, isLast }: { depth: number; isLast: boolean }) {
  return (
    <span className="flex items-stretch shrink-0 select-none" aria-hidden>
      {Array.from({ length: depth }, (_, i) => (
        <span key={i} className="relative w-5">
          <span className="absolute left-2.25 top-0 bottom-0 border-l border-foreground" />
        </span>
      ))}
      <span className="relative w-5">
        <span
          className={cn(
            "absolute left-2.25 border-l border-foreground",
            isLast ? "top-0 bottom-1/2" : "top-0 bottom-0",
          )}
        />
        <span className="absolute left-2.25 right-0.5 top-1/2 border-t border-foreground" />
      </span>
    </span>
  );
}
