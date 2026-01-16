import { ChevronDown, ChevronRight, ChevronUp } from "lucide-react";
import { cn } from "@/util";
import { DiffStatBar } from "./diff-stat-bar";

export function DiffHeader({
  open,
  setOpen,
  leftPath,
  rightPath,
  linesAdded,
  linesRemoved,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  leftPath: string | undefined;
  rightPath: string | undefined;
  linesAdded: number;
  linesRemoved: number;
}) {
  const path = leftPath || rightPath;

  return (
    <div
      className={cn(
        "flex flex-row w-full h-9 shrink-0 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10",
        open ? "bg-sidebar" : "bg-sidebar-primary border-b-transparent",
      )}
    >
      {leftPath && rightPath && leftPath !== rightPath ? (
        <span className="mr-auto">
          <span>{leftPath}</span>
          <span className="mx-1.25">{"→"}</span>
          {rightPath}
        </span>
      ) : (
        <span className="mr-auto">{path}</span>
      )}

      <button
        type="submit"
        className="flex flex-row items-center select-none"
        onClick={() => setOpen(!open)}
      >
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
            <DiffStatBar added={linesAdded} removed={linesRemoved} />
          ))}
        {open ? (
          <ChevronDown className="ml-1.5 mb-px size-3" />
        ) : (
          <ChevronRight className="ml-1.5 mb-px size-3" />
        )}
      </button>
    </div>
  );
}
