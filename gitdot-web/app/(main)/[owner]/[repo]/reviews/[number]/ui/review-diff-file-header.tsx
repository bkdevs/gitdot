import type {
  RepositoryDiffFileResource,
  RepositoryDiffStatResource,
} from "gitdot-api";

export function ReviewDiffFileHeader({
  diff,
}: {
  diff: RepositoryDiffStatResource | RepositoryDiffFileResource;
}) {
  const leftPath = diff.path;
  const rightPath = diff.path;
  const path = leftPath || rightPath;
  const { lines_added, lines_removed } = diff;

  return (
    <div
      data-diff-toggle
      id={path}
      className="flex flex-row w-full h-7 shrink-0 items-center px-2 text-xs font-mono bg-sidebar border-b border-border select-none"
    >
      {leftPath && rightPath && leftPath !== rightPath ? (
        <span className="mr-auto text-muted-foreground">
          <span>{leftPath}</span>
          <span className="mx-1.25">{"→"}</span>
          {rightPath}
        </span>
      ) : (
        <span data-diff-path className="mr-auto text-muted-foreground">
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
            <span className="font-mono select-none">
              <span className="text-green-600">+{lines_added}</span>
              <span className="mx-1" />
              <span className="text-red-600">-{lines_removed}</span>
            </span>
          ))}
      </div>
    </div>
  );
}
