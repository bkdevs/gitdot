"use client";

import type { UserCommitResource } from "gitdot-api";
import { useState } from "react";
import { formatIsoDate, subtractMonths } from "@/util/date";
import { UserCommitStatistics } from "./user-commit-statistics";
import { UserCommitsCalendar } from "./user-commits-calendar";
import { UserCommitsHeader } from "./user-commits-header";
import { UserCommitsLog } from "./user-commits-log";

export function UserCommits({ commits }: { commits: UserCommitResource[] }) {
  const [startDate, setStartDate] = useState(
    formatIsoDate(subtractMonths(new Date(), 11)),
  );
  const [endDate, setEndDate] = useState(formatIsoDate(new Date()));
  const [selectedMonth, setSelectedMonth] = useState<string | null>(null);

  const commitMap = new Map<string, UserCommitResource[]>();
  for (const c of commits) {
    const day = c.date.slice(0, 10);
    if (!commitMap.has(day)) commitMap.set(day, []);
    commitMap.get(day)?.push(c);
  }

  return (
    <div className="flex flex-col h-full min-h-0">
      <UserCommitsHeader
        startDate={startDate}
        endDate={endDate}
        setStartDate={setStartDate}
        setEndDate={setEndDate}
        setSelectedMonth={setSelectedMonth}
      />
      <UserCommitsCalendar
        commits={commitMap}
        startDate={startDate}
        endDate={endDate}
        selectedMonth={selectedMonth}
        setSelectedMonth={setSelectedMonth}
      />
      <div className="flex-1 min-h-0 overflow-y-auto scrollbar-none">
        <UserCommitStatistics
          commits={commitMap}
          startDate={startDate}
          endDate={endDate}
          selectedMonth={selectedMonth}
        />
        <UserCommitsLog
          commits={commitMap}
          startDate={startDate}
          endDate={endDate}
          selectedMonth={selectedMonth}
        />
      </div>
    </div>
  );
}
