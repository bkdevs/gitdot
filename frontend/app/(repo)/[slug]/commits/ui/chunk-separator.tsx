export function ChunkSeparator({
  "data-hidden-count": hiddenCount,
}: {
  "data-hidden-count": number;
}) {
  return (
    <div className="flex items-center w-full py-1 bg-muted/30 text-xs text-muted-foreground">
      <span className="w-9 text-right pr-1.5 mr-1 text-muted-foreground/60">
        ...
      </span>
      <span className="font-mono">
        {hiddenCount} line{hiddenCount !== 1 ? "s" : ""} hidden
      </span>
    </div>
  );
}
