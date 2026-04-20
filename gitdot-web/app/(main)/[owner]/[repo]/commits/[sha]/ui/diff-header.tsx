import { ChevronDown, ChevronRight } from "lucide-react";
import { cn } from "@/util";
import { DiffStatBar } from "./diff-stat-bar";

export function DiffHeader({
  open,
  setOpen,
  path,
  linesAdded,
  linesRemoved,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  path: string;
  linesAdded: number;
  linesRemoved: number;
}) {
  return (
    <button
      type="button"
      data-diff-toggle
      id={path}
      className={cn(
        "flex flex-row w-full h-9 shrink-0 items-center px-2 border-b border-border text-sm font-mono sticky top-0 z-10 select-none",
        open ? "bg-sidebar" : "bg-sidebar-primary",
      )}
      onClick={() => setOpen(!open)}
    >
      <span data-diff-path className="mr-auto">
        {path}
      </span>
      <div className="flex flex-row items-center">
        <DiffStatBar added={linesAdded} removed={linesRemoved} />
        {open ? (
          <ChevronDown className="ml-1.5 mb-px size-3" />
        ) : (
          <ChevronRight className="ml-1.5 mb-px size-3" />
        )}
      </div>
    </button>
  );
}
