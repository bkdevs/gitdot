"use client";

import { cn } from "@/util";
import { useFileBodyContext } from "./file-body-client";

export function FileLine({
  children,
  "data-line-number": lineNumber,
}: {
  children: React.ReactNode;
  "data-line-number": number;
}) {
  const { isLineSelected, handleLineMouseDown, handleLineMouseEnter } =
    useFileBodyContext();

  const isSelected = isLineSelected(lineNumber);

  return (
    <span
      className={cn("inline-flex w-full", isSelected && "bg-accent/60")}
      onMouseEnter={() => handleLineMouseEnter(lineNumber)}
    >
      <button
        type="button"
        className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none outline-none cursor-pointer"
        onMouseDown={() => handleLineMouseDown(lineNumber)}
      >
        {lineNumber}
      </button>
      {children}
    </span>
  );
}
