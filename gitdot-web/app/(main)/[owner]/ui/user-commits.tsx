"use client";

import { subtractMonths } from "@/util/date";
import type { RepositoryCommitResource } from "gitdot-api";
import { useState } from "react";
import { UserCommitsCalendar } from "./user-commits-calendar";
import { UserCommitsLog } from "./user-commits-log";
import { UserCommitsHeader } from "./user-commits-header";

export function UserCommits({
  commits,
}: {
  commits: RepositoryCommitResource[];
}) {
  const [startDate, _setStartDate] = useState(
    subtractMonths(new Date(), 11).toISOString().slice(0, 10),
  );
  const [endDate, _setEndDate] = useState(
    new Date().toISOString().slice(0, 10),
  );
  const [selectedMonth, setSelectedMonth] = useState<string | null>(null);

  const commitMap = new Map<string, RepositoryCommitResource[]>();
  for (const c of commits) {
    const day = c.date.slice(0, 10);
    if (!commitMap.has(day)) commitMap.set(day, []);
    commitMap.get(day)?.push(c);
  }

  return (
    <div>
      <UserCommitsHeader
      commits={commitMap}
      startDate={startDate}
      endDate={endDate}
      selectedMonth={selectedMonth}
      />
      <UserCommitsCalendar
      commits={commitMap}
      startDate={startDate}
      endDate={endDate}
      selectedMonth={selectedMonth}
      setSelectedMonth={setSelectedMonth}
      />
      <UserCommitsLog
      commits={commitMap}
      startDate={startDate}
      endDate={endDate}
      selectedMonth={selectedMonth}
      />
    </div>
  );
}
