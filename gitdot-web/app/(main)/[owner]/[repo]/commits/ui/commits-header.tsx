"use client";

import type { CommitFilterResource } from "gitdot-api";
import { ChevronDown, Plus } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { formatDate, subtractDays } from "@/util/date";
import { NewCommitFilterDialog } from "./new-commit-filter-dialog";

export function CommitsHeader({
  startDate,
  endDate,
  commitCount,
  filters,
  activeFilter,
  setActiveFilter,
  setStartDate,
  setEndDate,
}: {
  startDate: string | null;
  endDate: string | null;
  commitCount: number;
  filters: CommitFilterResource[];
  activeFilter: CommitFilterResource;
  setActiveFilter: (filter: CommitFilterResource) => void;
  setStartDate: (date: string | null) => void;
  setEndDate: (date: string | null) => void;
}) {
  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      {filters.map((filter) => (
        <FilterButton
          key={filter.name}
          filter={filter}
          isActive={activeFilter.name === filter.name}
          onClick={() => setActiveFilter(filter)}
        />
      ))}

      <NewFilterButton />
      <div className="ml-auto h-full flex flex-row">
        <DateRange
          startDate={startDate}
          endDate={endDate}
          commitCount={commitCount}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
        />
      </div>
    </div>
  );
}

function FilterButton({
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

function NewFilterButton() {
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

const DATE_OPTIONS = [
  { label: "Today", days: 0 },
  { label: "Last 7 days", days: 7 },
  { label: "Last 30 days", days: 30 },
  { label: "Last 90 days", days: 90 },
  { label: "Last 365 days", days: null },
] as const;

function toDateString(date: Date): string {
  return date.toISOString().slice(0, 10);
}

function DateRange({
  startDate,
  endDate,
  commitCount,
  setStartDate,
  setEndDate,
}: {
  startDate: string | null;
  endDate: string | null;
  commitCount: number;
  setStartDate: (date: string | null) => void;
  setEndDate: (date: string | null) => void;
}) {
  const end = endDate ? new Date(endDate) : new Date();
  const start = startDate ? new Date(startDate) : subtractDays(end, 365);
  const rangeLabel = `${formatDate(end)} — ${formatDate(start)}`;

  const today = toDateString(new Date());
  const activeDays =
    startDate === null
      ? null
      : (DATE_OPTIONS.find(
          (opt) =>
            opt.days !== null &&
            startDate === toDateString(subtractDays(new Date(), opt.days)) &&
            endDate === today,
        )?.days ?? -1);

  function select(days: number | null) {
    if (days === activeDays) {
      setStartDate(null);
      setEndDate(null);
      return;
    }
    if (days === null) {
      setStartDate(null);
      setEndDate(null);
    } else {
      setEndDate(today);
      setStartDate(toDateString(subtractDays(new Date(), days)));
    }
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex flex-row h-full items-center px-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground cursor-pointer select-none"
        >
          {rangeLabel} ({commitCount} commits)
          <ChevronDown className="size-3 ml-1" />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        {DATE_OPTIONS.map((opt) => (
          <DropdownMenuItem
            key={opt.label}
            className="text-xs"
            onSelect={() => select(opt.days)}
          >
            {opt.label}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
