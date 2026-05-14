"use client";

import type { CommitFilterResource } from "gitdot-api";
import { cn } from "@/util";

const MOCK_FILTERS: CommitFilterResource[] = [
  {
    name: "All commits",
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
  {
    name: "Backend commits",
    included_paths: ["gitdot-server/", "gitdot-core/"],
    excluded_paths: ["*.md"],
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
  {
    name: "Frontend commits",
    included_paths: ["gitdot-web/"],
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
  {
    name: "Commits by paul",
    authors: ["paul"],
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
  {
    name: "Migrations",
    included_paths: ["gitdot-core/migrations/"],
    tags: ["migration:"],
    created_at: "1970-01-01T00:00:00Z",
    updated_at: "1970-01-01T00:00:00Z",
  },
  {
    name: "Release commits",
    tags: ["release:", "v"],
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
      <div className="flex flex-col flex-1 min-h-0 overflow-y-auto">
        {MOCK_FILTERS.map((filter) => (
          <button
            key={filter.name}
            type="button"
            onClick={() => setActiveFilter(filter)}
            className={cn(
              "flex flex-row items-center h-8 px-2 text-sm text-left border-b border-border cursor-pointer transition-colors shrink-0",
              activeFilter.name === filter.name
                ? "bg-sidebar text-foreground"
                : "text-muted-foreground hover:bg-accent/50 hover:text-foreground",
            )}
          >
            {filter.name}
          </button>
        ))}
      </div>

      <FilterCriteria filter={active} />
    </div>
  );
}

function FilterCriteria({ filter }: { filter: CommitFilterResource }) {
  const rows: { label: string; values: string[] }[] = [];
  if (filter.authors?.length)
    rows.push({ label: "Authors", values: filter.authors });
  if (filter.tags?.length) rows.push({ label: "Tags", values: filter.tags });
  if (filter.included_paths?.length)
    rows.push({ label: "Included paths", values: filter.included_paths });
  if (filter.excluded_paths?.length)
    rows.push({ label: "Excluded paths", values: filter.excluded_paths });

  return (
    <div className="flex flex-col border-t border-border shrink-0">
      <div className="flex flex-row items-center h-8 px-2 text-xs text-muted-foreground border-b border-border">
        Filter criteria
      </div>
      {rows.length === 0 ? (
        <div className="px-2 py-2 text-xs text-muted-foreground">
          No criteria applied
        </div>
      ) : (
        rows.map((row) => (
          <CriteriaRow key={row.label} label={row.label} values={row.values} />
        ))
      )}
    </div>
  );
}

function CriteriaRow({ label, values }: { label: string; values: string[] }) {
  return (
    <div className="flex flex-col px-2 py-1.5">
      <span className="text-xs text-muted-foreground">{label}</span>
      <span className="text-xs text-foreground truncate">
        {values.join(", ")}
      </span>
    </div>
  );
}
