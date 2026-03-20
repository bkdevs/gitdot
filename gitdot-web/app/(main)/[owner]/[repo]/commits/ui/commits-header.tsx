"use client";

import type { CommitFilterResource } from "gitdot-api";
import { ChevronDown, Plus } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { formatDate } from "@/util/date";
import { NewCommitFilterDialog } from "./new-commit-filter-dialog";

export function CommitsHeader({
  startDate,
  endDate,
  commitCount,
  filters,
  activeFilter,
  setActiveFilter,
}: {
  startDate: string | null;
  endDate: string | null;
  commitCount: number;
  filters: CommitFilterResource[];
  activeFilter: CommitFilterResource;
  setActiveFilter: (filter: CommitFilterResource) => void;
}) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      {filters.map((filter) => (
        <CommitFilter
          key={filter.name}
          filter={filter}
          isActive={activeFilter.name === filter.name}
          onClick={() => setActiveFilter(filter)}
        />
      ))}

      <NewCommitFilterButton />
      <div className="ml-auto h-full flex flex-row">
        <DateRange
          startDate={startDate}
          endDate={endDate}
          commitCount={commitCount}
        />
      </div>
    </div>
  );
}

function CommitFilter({
  filter,
  isActive,
  onClick,
}: {
  filter: Pick<CommitFilterResource, "name">;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      className={cn(
        "flex flex-row items-center h-full border-border border-r px-2 text-xs hover:bg-sidebar",
        isActive ? "bg-sidebar text-foreground" : "text-muted-foreground",
      )}
      onClick={onClick}
    >
      {filter.name}
    </button>
  );
}

function NewCommitFilterButton() {
  const [open, setOpen] = useState(false);

  return (
    <>
      <button
        type="button"
        className="flex flex-row items-center h-full px-2 text-xs text-muted-foreground cursor-pointer transition-colors hover:text-foreground"
        onClick={() => setOpen(true)}
      >
        <Plus className="size-3 mr-1" />
        New filter
      </button>
      <NewCommitFilterDialog open={open} setOpen={setOpen} />
    </>
  );
}

function DateRange({
  startDate,
  endDate,
  commitCount,
}: {
  startDate: string | null;
  endDate: string | null;
  commitCount: number;
}) {
  const end = endDate ? new Date(endDate) : new Date();
  const start = startDate
    ? new Date(startDate)
    : new Date(new Date().setFullYear(end.getFullYear() - 1));
  const rangeLabel = `${formatDate(end)} — ${formatDate(start)}`;

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex flex-row h-full items-center px-2 text-xs text-muted-foreground transition-colors hover:text-foreground cursor-pointer select-none"
        >
          {rangeLabel} ({commitCount} commits)
          <ChevronDown className="size-3 ml-1" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel className="text-xs font-normal text-muted-foreground">
          Date range picker coming soon
        </DropdownMenuLabel>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
