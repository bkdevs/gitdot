import type { RepositoryFileDiff } from "@/lib/dto";

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
    <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
      {path}

      {leftPath && !rightPath && (
        <span className="ml-1.5 text-red-600">(deleted)</span>
      )}
      {!leftPath && rightPath && (
        <span className="ml-1.5 text-green-600">(created)</span>
      )}
    </div>
  );
}
