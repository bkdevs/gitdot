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
        "diff-line-row flex items-center w-full",
        "[&_.diff-token]:transition-colors",
        "[&_.diff-token]:duration-200",
        "[&_.diff-token:hover]:bg-primary/8",
        "[&_.diff-token.span-selected]:bg-primary/8",
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
