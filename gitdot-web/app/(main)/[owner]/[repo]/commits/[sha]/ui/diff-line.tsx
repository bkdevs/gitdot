import { cn } from "@/util";

export function DiffLine({
  children,
  "data-line-number": lineNumber,
  "data-line-type": lineType,
  "data-side": side,
}: {
  children: React.ReactNode;
  "data-line-number": number;
  "data-line-type": "sentinel" | "normal" | "added" | "removed";
  "data-side"?: string;
}) {
  return (
    <span
      className={cn(
        "diff-line",
        "flex items-center w-full",
        "[&_.diff-token]:cursor-pointer",
        "[&_.diff-token]:[transition:background-color_200ms]",
        "[&_.diff-token.token-active]:bg-diff-orange",
        "[&_.diff-token:hover,&_.diff-token.token-selected]:bg-primary/8",
        lineType === "added" && "bg-diff-green",
        lineType === "removed" && "bg-diff-red",
      )}
      data-line-number={lineType !== "sentinel" ? lineNumber : undefined}
      data-line-type={lineType}
      data-side={side}
    >
      <span className="w-7 text-right shrink-0 pr-1 mr-1 text-xs leading-5 text-primary/30 select-none">
        {lineType === "sentinel" ? ".." : lineNumber}
      </span>
      {children}
    </span>
  );
}
