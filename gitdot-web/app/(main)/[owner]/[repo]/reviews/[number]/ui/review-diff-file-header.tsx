import type { RepositoryDiffFileResource } from "gitdot-api";
import { Maximize2 } from "lucide-react";

export function ReviewDiffFileHeader({
  diff,
  onClick,
}: {
  diff: RepositoryDiffFileResource;
  onClick?: () => void;
}) {
  const { path, lines_added, lines_removed, left_content, right_content } =
    diff;
  const isCreated = !left_content;
  const isDeleted = !right_content;

  return (
    <div
      data-diff-toggle
      id={path}
      onClick={onClick}
      className="group flex flex-row w-full h-7 shrink-0 items-center px-2 text-xs font-mono bg-sidebar hover:bg-sidebar-accent/80 border-b border-border select-none cursor-pointer transition-colors duration-200"
    >
      <div className="flex flex-row items-center gap-2 mr-auto">
        <span data-diff-path className="text-muted-foreground">
          {path}
        </span>

        {isCreated && <span className="text-green-600">created</span>}
        {isDeleted && <span className="text-red-600">deleted</span>}
        {!isCreated && !isDeleted && (
          <span className="flex flex-row font-mono select-none gap-1">
            <span className="text-green-600">+{lines_added}</span>
            <span className="text-red-600">-{lines_removed}</span>
          </span>
        )}
      </div>

      <Maximize2 className="size-3 text-muted-foreground group-hover:text-foreground transition-colors duration-200 shrink-0" />
    </div>
  );
}
