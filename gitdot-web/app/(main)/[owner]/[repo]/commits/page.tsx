"use client";

import { useParams } from "next/navigation";
import { Suspense, use } from "react";
import Link from "@/ui/link";
import { timeAgoFull } from "@/util";
import { useRepoContext } from "../context";
import { CommitsHeader } from "./ui/commits-header";

export function CommitsClient() {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();

  const commits = use(useRepoContext().commits);
  if (!commits) return null;

  return (
    <div className="flex flex-col">
      <CommitsHeader />
      {commits.map((commit) => (
        <Link
          key={commit.sha}
          href={`/${owner}/${repo}/commits/${commit.sha.substring(0, 7)}`}
          data-page-item
          tabIndex={-1}
          className="flex w-full border-b focus:bg-accent/50 select-none cursor-default py-2 px-2 focus:outline-none"
          prefetch={true}
        >
          <div className="flex flex-col w-full justify-start items-start min-w-0">
            <div className="text-sm truncate mb-0.5 w-full">
              {commit.message}
            </div>
            <div className="text-xs text-muted-foreground flex items-center gap-1 w-full min-w-0">
              <span className="truncate min-w-0">{commit.author.name}</span>
              <span className="shrink-0">•</span>
              <span className="shrink-0">
                {timeAgoFull(new Date(commit.date))}
              </span>
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
}

export default function Page() {
  return (
    <Suspense>
      <CommitsClient />
    </Suspense>
  );
}
