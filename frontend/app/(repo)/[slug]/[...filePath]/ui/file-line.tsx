"use client";

export function FileLine({
  children,
  "data-line-number": lineNumber,
}: {
  children: React.ReactNode;
  "data-line-number": number;
}) {
  return (
    <span className="inline-flex">
      <span className="w-8 text-right shrink-0 pr-2 mr-3 text-primary/60 border-r select-none cursor-pointer">
        {lineNumber}
      </span>
      {children}
    </span>
  );
}
