import { cn } from "@/util";

export function DiffLine({
  children,
  "data-line-number": lineNumber,
  "data-line-type": lineType,
  "data-bg-color": bgColor,
}: {
  children: React.ReactNode;
  "data-line-number": number;
  "data-line-type": "sentinel" | "normal";
  "data-bg-color"?: string;
}) {
  return (
    <span
      className={cn("inline-flex w-full")}
      style={{ backgroundColor: bgColor }} // use inline styles to avoid tailwind JIT
    >
      <span className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none">
        {lineType === "sentinel" ? ".." : lineNumber}
      </span>
      {children}
    </span>
  );
}
