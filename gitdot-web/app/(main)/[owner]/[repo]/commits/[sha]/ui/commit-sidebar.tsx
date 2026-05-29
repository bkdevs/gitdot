"use client";

import type { DiffEntry } from "gitdot-dal/client";
import { cn } from "@/util";

export function CommitSidebar({
  entries,
  selectedIndex,
  onSelect,
}: {
  entries: DiffEntry[];
  selectedIndex: number;
  onSelect: (index: number) => void;
}) {
  if (entries.length === 0) return null;

  return (
    <div className="min-w-0">
      <div className="text-xs font-mono text-foreground mb-1">Files</div>
      <nav className="flex flex-col gap-1 font-mono text-xs">
        {entries.map((entry, i) => {
          const path = entry.resource.path;
          const name = path.split("/").slice(-2).join("/");
          const isCurrent = selectedIndex === i;
          return (
            <button
              key={path}
              type="button"
              title={path}
              onClick={() => {
                onSelect(i);
                document.getElementById(path)?.scrollIntoView();
              }}
              className={cn(
                "text-left truncate underline cursor-pointer",
                isCurrent
                  ? "text-foreground decoration-current"
                  : "text-muted-foreground decoration-transparent hover:text-foreground hover:decoration-current",
              )}
            >
              {name}
            </button>
          );
        })}
      </nav>
    </div>
  );
}
