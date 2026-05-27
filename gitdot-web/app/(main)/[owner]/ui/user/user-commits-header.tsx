"use client";

import { ChevronDown } from "lucide-react";
import { useTimezone } from "@/(main)/provider/timezone";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { formatDateIso, subtractMonths } from "@/util/date";

export function UserCommitsHeader({
  endDate,
  setStartDate,
  setEndDate,
  setSelectedMonth,
}: {
  startDate: string;
  endDate: string;
  setStartDate: (d: string) => void;
  setEndDate: (d: string) => void;
  setSelectedMonth: (m: string | null) => void;
}) {
  const tz = useTimezone();
  const currentYear = new Date().getFullYear();
  const displayYear = endDate.slice(0, 4);
  const years = [
    currentYear,
    currentYear - 1,
    currentYear - 2,
    currentYear - 3,
    currentYear - 4,
  ];

  function selectYear(y: number) {
    setSelectedMonth(null);
    if (y === currentYear) {
      setStartDate(subtractMonths(new Date(), 11, tz));
      setEndDate(formatDateIso(new Date(), tz));
    } else {
      setStartDate(`${y}-01-01`);
      setEndDate(`${y}-12-31`);
    }
  }

  return (
    <div className="flex items-baseline mb-2 justify-between px-3">
      <span className="text-xs text-muted-foreground font-mono">
        <span className="text-foreground/40 select-none"># </span>Commits
      </span>
      <DropdownMenu>
        <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs text-muted-foreground/60 font-mono cursor-pointer transition-colors hover:text-foreground">
          {displayYear}
          <ChevronDown className="size-3" />
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="min-w-20">
          {years.map((y) => (
            <DropdownMenuItem
              key={y}
              className="text-xs font-mono"
              onClick={() => selectYear(y)}
            >
              {y}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
