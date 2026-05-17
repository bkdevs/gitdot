"use client";

import { use, useEffect, useState } from "react";
import type { DiffEntry } from "@/actions";
import { cn } from "@/util";

export function CommitSidebar({
  diffEntriesPromise,
}: {
  diffEntriesPromise: Promise<DiffEntry[]>;
}) {
  const entries = use(diffEntriesPromise);
  const [currentIndex, setCurrentIndex] = useState(-1);

  useEffect(() => {
    if (entries.length === 0) return;

    const update = () => {
      const files = Array.from(
        document.querySelectorAll<HTMLElement>("[data-diff-file]"),
      );
      const viewportHeight = window.innerHeight;

      let bottomMostFullyVisible = -1;
      for (let i = 0; i < files.length; i++) {
        const rect = files[i].getBoundingClientRect();
        if (rect.top >= 0 && rect.bottom <= viewportHeight) {
          bottomMostFullyVisible = i;
        }
      }
      if (bottomMostFullyVisible !== -1) {
        setCurrentIndex(bottomMostFullyVisible);
        return;
      }

      let maxCoverage = 0;
      let current = -1;
      for (let i = 0; i < files.length; i++) {
        const rect = files[i].getBoundingClientRect();
        const visibleTop = Math.max(0, rect.top);
        const visibleBottom = Math.min(viewportHeight, rect.bottom);
        const coverage = Math.max(0, visibleBottom - visibleTop);
        if (coverage > maxCoverage) {
          maxCoverage = coverage;
          current = i;
        }
      }
      setCurrentIndex(current);
    };

    update();
    window.addEventListener("scroll", update, { capture: true, passive: true });
    window.addEventListener("wheel", update, { capture: true, passive: true });
    return () => {
      window.removeEventListener("scroll", update, { capture: true });
      window.removeEventListener("wheel", update, { capture: true });
    };
  }, [entries.length]);

  if (entries.length === 0) return null;

  return (
    <div className="min-w-0">
      <div className="text-xs font-mono text-foreground mb-1">Files</div>
      <nav className="flex flex-col gap-1 font-mono text-xs">
        {entries.map((entry, i) => {
          const path = entry.resource.path;
          const name = path.split("/").slice(-2).join("/");
          const isCurrent = currentIndex === i;
          return (
            <button
              key={path}
              type="button"
              title={path}
              onClick={() => document.getElementById(path)?.scrollIntoView()}
              className={cn(
                "text-left truncate underline cursor-pointer transition-colors duration-200",
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
