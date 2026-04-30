"use client";

import { useState } from "react";
import { cn } from "@/util";

type DiffView = "code" | "conversation";

export function ReviewDiffTabs() {
  const [view, setView] = useState<DiffView>("code");

  return (
    <div className="shrink-0 h-6 flex items-stretch gap-1">
      {(["code", "conversation"] as DiffView[]).map((v, i) => (
        <>
          {i > 0 && (
            <div key={`sep-${v}`} className="self-center w-px h-3 bg-border" />
          )}
          <button
            key={v}
            type="button"
            onClick={() => setView(v)}
            className={cn(
              "px-3 text-xs capitalize transition-colors border-t-2",
              view === v
                ? "text-foreground border-t-foreground"
                : "text-muted-foreground hover:text-foreground border-t-transparent",
            )}
          >
            {v}
          </button>
        </>
      ))}
    </div>
  );
}
