"use client";

import { Suspense, use } from "react";
import { useRepoContext } from "@/(main)/[owner]/[repo]/context";
import type { DiffData } from "@/actions/repository";
import { CommitBody } from "./commit-body";
import { CommitHeader } from "./commit-header";

export function CommitPageClient({
  sha,
  allDiffDataPromise,
}: {
  sha: string;
  allDiffDataPromise: Promise<Record<string, DiffData>>;
}) {
  return (
    <Suspense>
      <CommitPageInner sha={sha} allDiffDataPromise={allDiffDataPromise} />
    </Suspense>
  );
}

function CommitPageInner({
  sha,
  allDiffDataPromise,
}: {
  sha: string;
  allDiffDataPromise: Promise<Record<string, DiffData>>;
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
        <CommitBody
          diffs={commit.diffs}
          allDiffDataPromise={allDiffDataPromise}
        />
      </Suspense>
    </div>
  );
}
