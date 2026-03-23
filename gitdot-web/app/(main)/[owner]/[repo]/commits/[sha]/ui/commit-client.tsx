"use client";

import { Suspense, use } from "react";
import { useRepoContext } from "@/(main)/[owner]/[repo]/context";
import type { DiffEntry } from "@/actions";
import { Loading } from "@/ui/loading";
import { CommitBody } from "./commit-body";
import { CommitHeader } from "./commit-header";
import { CommitShortcuts } from "./commit-shortcuts";

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
    <div data-diff-top className="flex flex-col w-full">
      <CommitHeader commit={commit} stats={commit.diffs} />
      <Suspense fallback={<Loading />}>
        <CommitBody diffPromise={diffEntries} />
      </Suspense>
      <CommitShortcuts />
    </div>
  );
}
