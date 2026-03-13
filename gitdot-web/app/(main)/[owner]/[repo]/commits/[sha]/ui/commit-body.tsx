import type { RepositoryDiffResource } from "gitdot-api";
import { Suspense } from "react";
import { DiffBody } from "./diff-body";
import { DiffFileClient } from "./diff-file-client";

/**
 * we do the optional suspense to avoid the "loading" flicker for short loads, in which case
 * we just block the original page load instead.
 */
export function CommitBody({
  owner,
  repo,
  sha,
  parentSha,
  diffs,
  useSuspense,
}: {
  owner: string;
  repo: string;
  sha: string;
  parentSha: string | undefined;
  diffs: RepositoryDiffResource[];
  useSuspense: boolean;
}) {
  return useSuspense ? (
    <CommitSuspense>
      <CommitBodyContent
        owner={owner}
        repo={repo}
        sha={sha}
        parentSha={parentSha}
        diffs={diffs}
      />
    </CommitSuspense>
  ) : (
    <CommitBodyContent
      owner={owner}
      repo={repo}
      sha={sha}
      parentSha={parentSha}
      diffs={diffs}
    />
  );
}

function CommitBodyContent({
  owner,
  repo,
  sha,
  parentSha,
  diffs,
}: {
  owner: string;
  repo: string;
  sha: string;
  parentSha: string | undefined;
  diffs: RepositoryDiffResource[];
}) {
  return (
    <div className="flex flex-col">
      {diffs.slice(0, 3).map((stat) => (
        <DiffFileClient
          key={stat.path}
          leftPath={stat.path}
          rightPath={stat.path}
          linesAdded={stat.lines_added}
          linesRemoved={stat.lines_removed}
        >
          <CommitSuspense>
            <DiffBody
              stat={stat}
              owner={owner}
              repo={repo}
              sha={sha}
              parentSha={parentSha}
            />
          </CommitSuspense>
        </DiffFileClient>
      ))}
      {/* also use suspense to render large lists in two blocks, first three files for atf, and then after for btf */}
      <CommitSuspense>
        {diffs.slice(3).map((stat) => (
          <DiffFileClient
            key={stat.path}
            leftPath={stat.path}
            rightPath={stat.path}
            linesAdded={stat.lines_added}
            linesRemoved={stat.lines_removed}
          >
            <CommitSuspense>
              <DiffBody
                stat={stat}
                owner={owner}
                repo={repo}
                sha={sha}
                parentSha={parentSha}
              />
            </CommitSuspense>
          </DiffFileClient>
        ))}
      </CommitSuspense>
    </div>
  );
}

function CommitSuspense({ children }: { children: React.ReactNode }) {
  return (
    <Suspense
      fallback={
        // match padding of diff header to avoid layout shift
        <div className="flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground">
          loading...
        </div>
      }
    >
      {children}
    </Suspense>
  );
}
