import type {
  RepositoryDiffFileResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import { ChevronDown, ChevronRight } from "lucide-react";
import { cn } from "@/util";
import { DiffStatBar } from "./diff-stat-bar";

export function DiffHeader({
  open,
  setOpen,
  diff,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  diff: RepositoryDiffStatResource | RepositoryDiffFileResource;
}) {
  // TODO: renames no longer work
  const leftPath = diff.path;
  const rightPath = diff.path;
  const path = leftPath || rightPath;
  const { lines_added, lines_removed } = diff;

  return (
    <button
      type="button"
      id={path}
      className={cn(
        "flex flex-row w-full h-9 shrink-0 items-center px-2 border-b border-border text-sm font-mono sticky top-0 z-10 select-none",
        open ? "bg-sidebar" : "bg-sidebar-primary",
      )}
      onClick={() => setOpen(!open)}
    >
      {leftPath && rightPath && leftPath !== rightPath ? (
        <span className="mr-auto">
          <span>{leftPath}</span>
          <span className="mx-1.25">{"→"}</span>
          {rightPath}
        </span>
      ) : (
        <span data-diff-path className="mr-auto">
          {path}
        </span>
      )}

      <div className="flex flex-row items-center">
        {leftPath && !rightPath && (
          <span className="text-red-600">deleted</span>
        )}
        {!leftPath && rightPath && (
          <span className="text-green-600">created</span>
        )}
        {leftPath &&
          rightPath &&
          (leftPath !== rightPath ? (
            <span className="text-muted-foreground">renamed</span>
          ) : (
            <DiffStatBar added={lines_added} removed={lines_removed} />
          ))}
        {open ? (
          <ChevronDown className="ml-1.5 mb-px size-3" />
        ) : (
          <ChevronRight className="ml-1.5 mb-px size-3" />
        )}
      </div>
    </button>
  );
}
