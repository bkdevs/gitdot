"use client";

import { Suspense, use } from "react";
import { useRepoContext } from "@/(main)/[owner]/[repo]/context";
import type { DiffEntry } from "@/actions";
import { CommitBody } from "./commit-body";
import { CommitHeader } from "./commit-header";

export function CommitClient({
  sha,
  diffEntries,
}: {
  sha: string;
  diffEntries: Promise<DiffEntry[]>;
}) {
  const commits = use(useRepoContext().commits);
  const commit = commits?.find((c) => c.sha.startsWith(sha));

  if (!commit) return null;

  return (
    <div className="flex flex-col w-full">
      <CommitHeader commit={commit} stats={commit.diffs} />
      <Suspense
        fallback={
          // match padding of diff header to avoid layout shift
          <div className="flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground">
            loading...
          </div>
        }
      >
        <CommitBody diffEntries={diffEntries} />
      </Suspense>
    </div>
  );
}
