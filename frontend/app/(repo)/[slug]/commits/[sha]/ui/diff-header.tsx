import { ChevronDown } from "lucide-react";
import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffStatBar } from "./diff-stat-bar";

export function DiffHeader({
  leftPath,
  rightPath,
  linesAdded,
  linesRemoved,
}: {
  leftPath: string | undefined;
  rightPath: string | undefined;
  linesAdded: number;
  linesRemoved: number;
}) {
  const path = leftPath || rightPath;

  return (
    <div className="flex flex-row w-full h-9 shrink-0 items-center px-2 border-t border-b border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
      <span className="mr-auto">{path}</span>

      {leftPath && !rightPath && <span className="text-red-600">deleted</span>}
      {!leftPath && rightPath && (
        <span className="text-green-600">created</span>
      )}
      {leftPath && rightPath && (
        <DiffStatBar added={linesAdded} removed={linesRemoved} />
      )}
      <ChevronDown className="ml-1.5 size-3" />
    </div>
  );
}
