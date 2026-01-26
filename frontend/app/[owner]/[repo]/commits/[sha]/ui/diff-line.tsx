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
        "inline-flex w-full",
        lineType === "added" && "bg-diff-green",
        lineType === "removed" && "bg-diff-red",
      )}
    >
      <span className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none">
        {lineType === "sentinel" ? ".." : lineNumber}
      </span>
      {children}
    </span>
  );
}
