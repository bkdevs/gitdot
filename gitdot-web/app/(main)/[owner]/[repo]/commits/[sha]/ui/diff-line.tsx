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
        "inline-flex items-center w-full",
        lineType === "added" && "bg-diff-green",
        lineType === "removed" && "bg-diff-red",
      )}
    >
      <span className="w-7 text-right shrink-0 pr-1 mr-1 text-xs leading-5 text-primary/30 select-none">
        {lineType === "sentinel" ? ".." : lineNumber}
      </span>
      {children}
    </span>
  );
}
