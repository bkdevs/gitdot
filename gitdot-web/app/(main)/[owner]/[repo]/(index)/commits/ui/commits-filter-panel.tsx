"use client";

import type { CommitFilterResource } from "gitdot-api";
import { cn } from "@/util";

const MOCK_FILTERS: CommitFilterResource[] = [
  {
    name: "baepaul",
    authors: ["baepaul"],
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
  {
    name: "mikkel",
    authors: ["mikkel"],
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
];

export function CommitsFilterPanel({
  filters: _filters,
  activeFilter,
  setActiveFilter,
}: {
  filters: CommitFilterResource[];
  activeFilter: CommitFilterResource;
  setActiveFilter: (filter: CommitFilterResource) => void;
}) {
  const active =
    MOCK_FILTERS.find((f) => f.name === activeFilter.name) ?? MOCK_FILTERS[0];

  return (
    <div className="flex flex-col w-1/4 border-l border-border">
      <FilterDetail filter={active} />
      <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
        <div className="flex items-center h-8 px-2 shrink-0">
          <span className="text-xs text-muted-foreground">Filters</span>
        </div>
        {MOCK_FILTERS.map((filter) => (
          <button
            key={filter.name}
            type="button"
            onClick={() => setActiveFilter(filter)}
            className={cn(
              "flex flex-row items-center h-7 px-2 text-xs text-left transition-colors shrink-0",
              activeFilter.name === filter.name
                ? "bg-sidebar text-foreground border-l-2 border-foreground"
                : "text-muted-foreground hover:bg-accent/50 hover:text-foreground border-l-2 border-transparent",
            )}
          >
            {filter.name}
          </button>
        ))}
        <div className="border-b border-border shrink-0" />
      </div>
    </div>
  );
}

function FilterDetail({ filter }: { filter: CommitFilterResource }) {
  const rows: { label: string; values: string[] }[] = [];
  if (filter.authors?.length)
    rows.push({ label: "Authors", values: filter.authors });
  if (filter.tags?.length) rows.push({ label: "Tags", values: filter.tags });
  if (filter.included_paths?.length)
    rows.push({ label: "Paths", values: filter.included_paths });
  if (filter.excluded_paths?.length)
    rows.push({ label: "Excluded", values: filter.excluded_paths });

  return (
    <div className="flex flex-col h-42 shrink-0 px-3 py-2.5 gap-1.5 overflow-hidden border-b border-border">
      <span className="text-sm font-medium text-foreground truncate">
        {filter.name}
      </span>
      {rows.length === 0 ? (
        <span className="text-xs text-muted-foreground">No criteria applied</span>
      ) : (
        <div className="flex flex-col gap-1 min-h-0 overflow-hidden">
          {rows.map((row) => (
            <div key={row.label} className="flex flex-col min-w-0">
              <span className="text-[10px] text-muted-foreground uppercase tracking-wide">
                {row.label}
              </span>
              <span className="text-xs text-foreground truncate">
                {row.values.join(", ")}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
