import { cn } from "@/util";

export function DiffLine({
  children,
  "data-line-number": lineNumber,
}: {
  children: React.ReactNode;
  "data-line-number": number;
}) {
  return (
    <span className={cn("inline-flex w-full")}>
      <span className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none">
        {lineNumber}
      </span>
      {children}
    </span>
  );
}
