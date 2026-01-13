"use client";

import React from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { groupCommitsByDate, formatDateHeader, formatTime } from "@/util";

const EXAMPLE_COMMITS = [
  {
    sha: "a1b2c3d4e5f6789",
    message: "Fix authentication bug in login flow",
    author: "Alice Johnson",
    date: "2024-01-10T14:30:00Z",
  },
  {
    sha: "b2c3d4e5f6a7890",
    message: "Add dark mode support",
    author: "Bob Smith",
    date: "2024-01-09T11:20:00Z",
  },
  {
    sha: "c3d4e5f6a7b8901",
    message: "Improve performance of data grid",
    author: "Charlie Davis",
    date: "2024-01-08T16:45:00Z",
  },
  {
    sha: "d4e5f6a7b8c9012",
    message: "Update dependencies to latest versions",
    author: "Diana Miller",
    date: "2024-01-07T09:15:00Z",
  },
  {
    sha: "e5f6a7b8c9d0123",
    message: "Add unit tests for API endpoints",
    author: "Eve Wilson",
    date: "2024-01-06T13:30:00Z",
  },
  {
    sha: "f6a7b8c9d0e1234",
    message: "Refactor component structure",
    author: "Frank Moore",
    date: "2024-01-05T10:00:00Z",
  },
  {
    sha: "a7b8c9d0e1f2345",
    message: "Fix mobile responsiveness issues",
    author: "Grace Taylor",
    date: "2024-01-04T15:20:00Z",
  },
  {
    sha: "b8c9d0e1f2a3456",
    message: "Add TypeScript strict mode",
    author: "Henry Anderson",
    date: "2024-01-03T12:45:00Z",
  },
  {
    sha: "c9d0e1f2a3b4567",
    message: "Implement user profile page",
    author: "Ivy Thomas",
    date: "2024-01-02T17:30:00Z",
  },
  {
    sha: "d0e1f2a3b4c5678",
    message: "Fix memory leak in dashboard",
    author: "Jack Jackson",
    date: "2024-01-01T08:15:00Z",
  },
];

function DateHeader({ dateKey }: { dateKey: string }) {
  return (
    <div className="sticky top-0 bg-background border-b px-2 py-1.5 z-10">
      <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
        {formatDateHeader(dateKey)}
      </h3>
    </div>
  );
}

export function RepoSidebarCommits() {
  const params = useParams();
  const slug = params.slug as string;

  // Group commits by date with memoization
  const groupedCommits = React.useMemo(
    () => groupCommitsByDate(EXAMPLE_COMMITS),
    [], // TODO: Add real commits dependency when dynamic
  );

  return (
    <div className="flex flex-col">
      {groupedCommits.map(([dateKey, commits]) => (
        <React.Fragment key={dateKey}>
          <DateHeader dateKey={dateKey} />
          {commits.map((commit) => (
            <Link
              key={commit.sha}
              href={`/${slug}/commits/${commit.sha.substring(0, 7)}`}
              className="flex w-full border-b hover:bg-accent/50 select-none cursor-default py-2 px-2"
            >
              <div className="flex flex-col w-full justify-start items-start min-w-0">
                <div className="text-sm truncate mb-0.5 w-full">
                  {commit.message}
                </div>
                <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
                  <span className="truncate min-w-0">{commit.author}</span>
                  <span className="shrink-0">•</span>
                  <span className="shrink-0">
                    {formatTime(new Date(commit.date))}
                  </span>
                </div>
              </div>
            </Link>
          ))}
        </React.Fragment>
      ))}
    </div>
  );
}
