"use client";

import { cn } from "@/util";
import { useFileViewer } from "./file-viewer-client";

export function FileLine({
  children,
  "data-line-number": lineNumber,
}: {
  children: React.ReactNode;
  "data-line-number": number;
}) {
  const { isLineSelected, handleLineMouseDown, handleLineMouseEnter } =
    useFileViewer();

  const isSelected = isLineSelected(lineNumber);

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: hover interaction for capturing selections
    <span
      className={cn("inline-flex w-full", isSelected && "bg-accent/80")}
      onMouseEnter={() => handleLineMouseEnter(lineNumber)}
    >
      <button
        type="button"
        className="w-8 text-right shrink-0 pr-2 mr-1 text-primary/60 border-r select-none outline-none cursor-pointer"
        onMouseDown={() => handleLineMouseDown(lineNumber)}
      >
        {lineNumber}
      </button>
      {children}
    </span>
  );
}
