import type { DiffResource } from "gitdot-api";
import { Suspense } from "react";
import { DiffBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-body";
import { DiffFileClient } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-file-client";
import { getReviewDiff } from "@/dal";

export async function ReviewDiffContent({
  owner,
  repo,
  number,
  diff,
}: {
  owner: string;
  repo: string;
  number: number;
  diff: DiffResource;
}) {
  const revision = diff.revisions[0];
  if (!revision) {
    return (
      <p className="text-sm text-muted-foreground px-4 py-2">No revisions</p>
    );
  }

  const diffResponse = await getReviewDiff(owner, repo, number, diff.position);
  if (!diffResponse) {
    return (
      <p className="text-sm text-muted-foreground px-4 py-2">
        Failed to load diff
      </p>
    );
  }

  return (
    <div className="flex flex-col">
      {diffResponse.files.map((stat) => (
        <DiffFileClient
          key={stat.path}
          leftPath={stat.path}
          rightPath={stat.path}
          linesAdded={stat.lines_added}
          linesRemoved={stat.lines_removed}
        >
          <Suspense
            fallback={
              <div className="flex flex-row w-full h-9 shrink-0 items-center p-2 font-mono text-sm text-muted-foreground">
                loading...
              </div>
            }
          >
            <DiffBody
              stat={stat}
              owner={owner}
              repo={repo}
              sha={revision.commit_hash}
              parentSha={revision.parent_hash}
            />
          </Suspense>
        </DiffFileClient>
      ))}
    </div>
  );
}
