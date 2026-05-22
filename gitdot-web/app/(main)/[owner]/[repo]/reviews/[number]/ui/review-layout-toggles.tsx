"use client";

import { PanelLeftOpen, PanelRightOpen } from "lucide-react";
import { cn } from "@/util";
import type { PageLayout } from "../page.client";

export function ReviewLayoutToggles({
  layout,
  setLayout,
}: {
  layout: PageLayout;
  setLayout: (layout: PageLayout) => void;
}) {
  return (
    <div className="flex flex-row w-16 h-8 border-t border-l border-border bg-background">
      <button
        type="button"
        onClick={() => setLayout(layout === "diffs" ? "split" : "diffs")}
        className={cn(
          "flex flex-1 items-center justify-center border-r border-border transition-colors",
          layout === "diffs"
            ? "bg-primary text-primary-foreground"
            : "text-muted-foreground hover:text-foreground",
        )}
      >
        <PanelRightOpen className="size-3.5" />
      </button>
      <button
        type="button"
        onClick={() => setLayout(layout === "summary" ? "split" : "summary")}
        className={cn(
          "flex flex-1 items-center justify-center transition-colors",
          layout === "summary"
            ? "bg-primary text-primary-foreground"
            : "text-muted-foreground hover:text-foreground",
        )}
      >
        <PanelLeftOpen className="size-3.5" />
      </button>
    </div>
  );
}
