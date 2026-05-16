"use client";

import type { RepositoryCommitFilterResource } from "gitdot-api";
import { cn } from "@/util";

export function CommitsFilterList({
  filters,
  activeFilter,
  setActiveFilter,
  isModified,
}: {
  filters: RepositoryCommitFilterResource[];
  activeFilter: RepositoryCommitFilterResource;
  setActiveFilter: (filter: RepositoryCommitFilterResource) => void;
  isModified: boolean;
}) {
  return (
    <div className="flex flex-col h-42 shrink-0 border-b border-border">
      <div className="flex items-center h-6 px-2 shrink-0 border-b border-border">
        <span className="text-xs text-muted-foreground font-mono">Filters</span>
      </div>
      <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
        {filters.map((filter) => {
          const isActive = activeFilter.id === filter.id;
          return (
            <button
              key={filter.id}
              type="button"
              onClick={() => setActiveFilter(filter)}
              className={cn(
                "w-full flex flex-row items-center h-6 px-2 text-xs text-left transition-colors shrink-0 border-b border-border font-mono",
                isActive
                  ? "bg-accent text-foreground"
                  : "text-muted-foreground hover:bg-accent/50 hover:text-foreground",
              )}
            >
              {filter.name}
              {isActive && isModified ? " (*)" : ""}
            </button>
          );
        })}
      </div>
    </div>
  );
}
