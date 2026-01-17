import { Suspense } from "react";
import { getRepositoryCommitDiffs } from "@/lib/dal";
import { CommitHeader } from "./ui/commit-header";
import { DiffBody } from "./ui/diff-body";
import { DiffFileClient } from "./ui/diff-file-client";

export default async function Page({
  params,
}: {
  params: Promise<{ slug: string; sha: string }>;
}) {
  const { slug: repo, sha } = await params;
  const commitDiffs = await getRepositoryCommitDiffs("bkdevs", repo, sha);
  if (!commitDiffs) return null;
  const { commit, diffs } = commitDiffs;

  return (
    <div className="flex flex-col w-full h-screen overflow-y-auto scrollbar-thin">
      <CommitHeader commit={commit} diffs={diffs} />
      <div className="flex flex-col">
        {diffs.slice(0, 2).map((diff) => {
          const key = diff.left?.path || diff.right?.path;
          return (
            <DiffFileClient
              key={key}
              leftPath={diff.left?.path}
              rightPath={diff.right?.path}
              linesAdded={diff.lines_added}
              linesRemoved={diff.lines_removed}
            >
              <DiffBody diff={diff} />
            </DiffFileClient>
          );
        })}
        {diffs.length > 2 && (
          <Suspense
            fallback={
              <div className="text-muted-foreground font-mono text-sm px-2">
                loading...
              </div>
            }
          >
            {diffs.slice(3).map((diff) => {
              const key = diff.left?.path || diff.right?.path;
              return (
                <DiffFileClient
                  key={key}
                  leftPath={diff.left?.path}
                  rightPath={diff.right?.path}
                  linesAdded={diff.lines_added}
                  linesRemoved={diff.lines_removed}
                >
                  <DiffBody diff={diff} />
                </DiffFileClient>
              );
            })}
          </Suspense>
        )}
      </div>
    </div>
  );
}
