import { cn } from "@/util";

export function DiffLine({
  children,
  "data-line-number": lineNumber,
  "data-line-type": lineType,
}: {
  children: React.ReactNode;
  "data-line-number": number;
  "data-line-type": "sentinel" | "normal" | "added" | "removed";
}) {
  return (
    <span
      className={cn(
        "flex items-center w-full",
        "[&_span:not(.diff-gutter)]:underline",
        "[&_span:not(.diff-gutter)]:decoration-transparent",
        "[&_span:not(.diff-gutter)]:transition-colors",
        "[&_span:not(.diff-gutter)]:duration-200",
        "[&_span:not(.diff-gutter):hover]:decoration-current",
        lineType === "added" && "bg-diff-green",
        lineType === "removed" && "bg-diff-red",
      )}
    >
      <span className="diff-gutter w-7 text-right shrink-0 pr-1 mr-1 text-xs leading-5 text-primary/30 select-none">
        {lineType === "sentinel" ? ".." : lineNumber}
      </span>
      {children}
    </span>
  );
}
