"use client";

import Link from "@/ui/link";
import { cn } from "@/util";
import type { FolderTreeRow } from "../util";
export type { FolderTreeRow as TreeRowData } from "../util";

export function FolderTreeRow({
  row,
  owner,
  repo,
  focused,
  active,
  onMouseEnter,
  onClick,
  absolutePaths,
}: {
  row: FolderTreeRow;
  owner: string;
  repo: string;
  focused: boolean;
  active: boolean;
  onMouseEnter: () => void;
  onClick: (path: string) => void;
  absolutePaths: boolean;
}) {
  return row.isTree ? (
    <TreeRowFolder
      row={row}
      owner={owner}
      repo={repo}
      focused={focused}
      active={active}
      onMouseEnter={onMouseEnter}
      onClick={onClick}
      absolutePaths={absolutePaths}
    />
  ) : (
    <TreeRowFile
      row={row}
      owner={owner}
      repo={repo}
      focused={focused}
      active={active}
      onMouseEnter={onMouseEnter}
      absolutePaths={absolutePaths}
    />
  );
}

function TreeRowFolder({
  row,
  owner,
  repo,
  focused,
  active,
  onMouseEnter,
  onClick,
  absolutePaths,
}: {
  row: FolderTreeRow;
  owner: string;
  repo: string;
  focused: boolean;
  active: boolean;
  onMouseEnter: () => void;
  onClick: (path: string) => void;
  absolutePaths: boolean;
}) {
  return (
    <div
      className={cn(
        "flex items-stretch gap-1 font-mono text-sm h-6 shrink-0 select-none w-full pl-1 pr-2",
        focused && "bg-sidebar",
      )}
      onMouseEnter={onMouseEnter}
      onClick={() => onClick(row.path)}
    >
      <TreeRowGutter depth={row.depth} isLast={row.isLast} />
      <Link
        href={`/${owner}/${repo}/${row.path}`}
        className={cn("flex items-center cursor-pointer hover:underline", active && "underline")}
        onClick={(e) => e.stopPropagation()}
      >
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        {row.name}
        {row.isExpanded && "/"}
        {!row.isExpanded && (
          <span className="ml-1 text-xs text-muted-foreground inline-flex items-center">
            ({row.fileCount})
          </span>
        )}
      </Link>
    </div>
  );
}

export function TreeRowFile({
  row,
  owner,
  repo,
  focused,
  active,
  onMouseEnter,
  absolutePaths,
}: {
  row: FolderTreeRow;
  owner: string;
  repo: string;
  focused: boolean;
  active: boolean;
  onMouseEnter: () => void;
  absolutePaths: boolean;
}) {
  return (
    <div
      data-path={row.path}
      className={cn(
        "flex items-stretch gap-1 font-mono text-sm h-6 shrink-0 select-none px-1 w-full",
        focused && "bg-sidebar",
      )}
      onMouseEnter={onMouseEnter}
    >
      <TreeRowGutter depth={row.depth} isLast={row.isLast} />
      <Link
        href={`/${owner}/${repo}/${row.path}`}
        className={cn("flex items-center cursor-pointer hover:underline", active && "underline")}
      >
        {absolutePaths && (
          <span className="text-muted-foreground">
            {row.path.split("/").slice(0, -1).join("/")}/
          </span>
        )}
        {row.name}
      </Link>
    </div>
  );
}

function TreeRowGutter({ depth, isLast }: { depth: number; isLast: boolean }) {
  return (
    <span className="flex items-stretch shrink-0 select-none" aria-hidden>
      {Array.from({ length: depth }, (_, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: stable positional slots
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
