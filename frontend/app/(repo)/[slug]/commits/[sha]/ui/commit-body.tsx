import { Suspense } from "react";
import { getRepositoryCommitDiffs } from "@/lib/dal";
import { DiffBody } from "./diff-body";
import { DiffFileClient } from "./diff-file-client";

/**
 * we do the optional suspense to avoid the "loading" flicker for short loads, in which case
 * we just block the original page load instead.
 */
export async function CommitBody({
  repo,
  sha,
  useSuspense,
}: {
  repo: string;
  sha: string;
  useSuspense: boolean;
}) {
  return useSuspense ? (
    <CommitSuspense>
      <CommitBodyContent repo={repo} sha={sha} />
    </CommitSuspense>
  ) : (
    <CommitBodyContent repo={repo} sha={sha} />
  );
}

async function CommitBodyContent({ repo, sha }: { repo: string; sha: string }) {
  const commitDiffs = await getRepositoryCommitDiffs("bkdevs", repo, sha);
  if (!commitDiffs) return null;
  const { diffs } = commitDiffs;

  return (
    <div className="flex flex-col">
      {diffs.slice(0, 3).map((diff) => {
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
      {/* also use suspense to render large lists in two blocks, first three files for atf, and then after for btf */}
      <CommitSuspense>
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
      </CommitSuspense>
    </div>
  );
}

function CommitSuspense({ children }: { children: React.ReactNode }) {
  return (
    <Suspense
      fallback={
        // match padding of diff header to avoid layout shift
        <div className="flex flex-row w-full h-9 shrink-0 items-center p-2 border-border border-t font-mono text-sm text-muted-foreground">
          loading...
        </div>
      }
    >
      {children}
    </Suspense>
  );
}
