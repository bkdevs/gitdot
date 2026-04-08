"use client";

import { ChevronDownIcon } from "lucide-react";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import { ActivityGrid } from "./activity-grid";

function inRange(date: string, start: string | null, end: string | null): boolean {
  if (!start || !end) return false;
  const lo = start <= end ? start : end;
  const hi = start <= end ? end : start;
  return date >= lo && date <= hi;
}

type FakeCommit = {
  sha: string;
  repo: string;
  message: string;
  author: { name: string; email: string };
  diffs: { path: string; lines_added: number; lines_removed: number }[];
};

const FAKE_LOG: Record<number, FakeCommit[]> = {
  0: [
    {
      sha: "1ff1b56", repo: "gitdot", message: "renamed app_state -> state",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [{ path: "gitdot-server/src/state.rs", lines_added: 12, lines_removed: 12 }],
    },
    {
      sha: "afb7e95", repo: "gitdot", message: "replaced ip nutype with IpNetwork",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [
        { path: "gitdot-core/src/models/device.rs", lines_added: 8, lines_removed: 14 },
        { path: "Cargo.toml", lines_added: 1, lines_removed: 2 },
      ],
    },
  ],
  1: [
    {
      sha: "6f8319b", repo: "gitdot", message: "updated ip extractor to work for local dev",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [{ path: "gitdot-server/src/extractor/ip.rs", lines_added: 19, lines_removed: 4 }],
    },
    {
      sha: "01baa6a", repo: "gitdot", message: "updated cli client with auth server url",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [{ path: "gitdot-cli/src/client.rs", lines_added: 6, lines_removed: 3 }],
    },
    {
      sha: "5a22349", repo: "gitdot", message: "updated authorizeDevice to call auth server",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [
        { path: "gitdot-core/src/services/device.rs", lines_added: 34, lines_removed: 11 },
        { path: "gitdot-core/src/services/auth.rs", lines_added: 5, lines_removed: 0 },
      ],
    },
  ],
  2: [
    {
      sha: "c3d82f1", repo: "s2-sdk-rs", message: "add retry logic for stream append",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [
        { path: "s2-sdk/src/client.rs", lines_added: 47, lines_removed: 9 },
        { path: "s2-sdk/src/error.rs", lines_added: 12, lines_removed: 2 },
      ],
    },
  ],
  3: [],
  4: [
    {
      sha: "b91ac44", repo: "gitdot", message: "wire up device auth flow in server",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [
        { path: "gitdot-server/src/routes/auth.rs", lines_added: 61, lines_removed: 4 },
        { path: "gitdot-server/src/routes/mod.rs", lines_added: 3, lines_removed: 0 },
      ],
    },
    {
      sha: "d04e712", repo: "gitdot", message: "add auth_code expiry to device table",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [{ path: "gitdot-core/migrations/0014_device_auth_expiry.sql", lines_added: 7, lines_removed: 0 }],
    },
  ],
  5: [
    {
      sha: "f3a19c0", repo: "s2-sdk-rs", message: "fix off-by-one in sequence window",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [{ path: "s2-sdk/src/sequence.rs", lines_added: 3, lines_removed: 3 }],
    },
  ],
  6: [
    {
      sha: "e72b381", repo: "dotfiles", message: "update zsh aliases",
      author: { name: "pybae", email: "paul@gitdot.io" },
      diffs: [{ path: ".zshrc", lines_added: 5, lines_removed: 1 }],
    },
  ],
};

export function UserCommits({ owner }: { owner: string }) {
  const [startDate, setStartDate] = useState<string | null>(null);
  const [endDate, setEndDate] = useState<string | null>(null);
  const currentYear = new Date().getFullYear();
  const [selectedYear, setSelectedYear] = useState(currentYear);
  const years = [currentYear, currentYear - 1, currentYear - 2];

  const logDays = Array.from({ length: 7 }, (_, i) => {
    const d = new Date();
    d.setDate(d.getDate() - i);
    return { date: d.toISOString().slice(0, 10), commits: FAKE_LOG[i] ?? [] };
  });

  const visibleDays =
    startDate && endDate
      ? logDays.filter(({ date }) => inRange(date, startDate, endDate))
      : logDays;

  return (
    <>
      <div>
        <div className="flex items-baseline justify-between mb-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>Activity
          </p>
          <DropdownMenu>
            <DropdownMenuTrigger className="flex items-center gap-0.5 text-xs font-mono text-muted-foreground hover:text-foreground transition-colors">
              {selectedYear}
              <ChevronDownIcon className="size-3" />
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              {years.map((y) => (
                <DropdownMenuItem
                  key={y}
                  className={y === selectedYear ? "text-foreground" : ""}
                  onSelect={() => { setSelectedYear(y); setStartDate(null); setEndDate(null); }}
                >
                  {y}
                </DropdownMenuItem>
              ))}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
        <ActivityGrid
          startDate={startDate}
          endDate={endDate}
          setStartDate={setStartDate}
          setEndDate={setEndDate}
        />
      </div>

      <div>
        <div className="flex items-baseline justify-between mb-2">
          <p className="text-xs text-muted-foreground font-mono">
            <span className="text-foreground/40 select-none"># </span>Log
          </p>
          <span className="text-xs font-mono text-muted-foreground/60">
            {(startDate ?? new Date().toISOString()).slice(0, 7)} ({visibleDays.reduce((n, d) => n + d.commits.length, 0)} commits)
          </span>
        </div>
        <div className="flex flex-col gap-4">
          {visibleDays.map(({ date, commits }) => (
            <div key={date}>
              <p className="text-xs text-muted-foreground font-mono mb-2">
                <span className="text-foreground/40 select-none">## </span>
                {date}
              </p>
              {commits.length === 0 ? (
                <p className="text-xs text-muted-foreground/50 font-mono">—</p>
              ) : (
                <div className="flex flex-col gap-1.5">
                  {commits.map((c) => {
                    const added = c.diffs.reduce((s, d) => s + d.lines_added, 0);
                    const removed = c.diffs.reduce((s, d) => s + d.lines_removed, 0);
                    return (
                      <div key={c.sha} className="flex items-baseline gap-2">
                        <span className="text-xs font-mono text-muted-foreground shrink-0">{c.sha}</span>
                        <span className="text-xs text-muted-foreground shrink-0">{c.repo}</span>
                        <span className="text-xs flex-1">{c.message}</span>
                        <span className="text-xs font-mono text-muted-foreground/50 shrink-0">{c.diffs.length} files</span>
                        <span className="text-xs font-mono text-green-600 dark:text-green-500 shrink-0">+{added}</span>
                        <span className="text-xs font-mono text-red-600 dark:text-red-500 shrink-0">-{removed}</span>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
