"use client";

import { ChevronDown } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { cn } from "@/util";
import { formatDate } from "@/util/date";

const MOCK_TAGS = ["Frontend", "Backend"];

export function CommitsHeader({
  startDate,
  endDate,
  commitCount,
}: {
  startDate: string | null;
  endDate: string | null;
  commitCount: number;
}) {
  const [activeTags, setActiveTags] = useState<Set<string>>(new Set());

  function toggleTag(tag: string) {
    setActiveTags((prev) => {
      const next = new Set(prev);
      if (next.has(tag)) next.delete(tag);
      else next.add(tag);
      return next;
    });
  }

  return (
    <div className="flex flex-row w-full h-9 items-center border-b">
      {MOCK_TAGS.map((tag) => (
        <TagButton
          key={tag}
          label={tag}
          isActive={activeTags.has(tag)}
          onClick={() => toggleTag(tag)}
        />
      ))}
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

function TagButton({
  label,
  isActive,
  onClick,
}: {
  label: string;
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
      {label}
    </button>
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
