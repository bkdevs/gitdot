"use client";

import type { ReviewDiffResource, ReviewResource } from "gitdot-api";
import { use } from "react";
import type { DiffEntry } from "@/actions";
import { ReviewDiffActions } from "./review-diff-actions";
import { ReviewDiffFile } from "./review-diff-file";
import { ReviewDiffMessage } from "./review-diff-message";

export function ReviewDiffBody({
  owner,
  repo,
  review,
  diffEntriesPromise,
  diff,
}: {
  owner: string;
  repo: string;
  review: ReviewResource;
  diffEntriesPromise: Promise<DiffEntry[]>;
  diff: ReviewDiffResource;
}) {
  const entries = use(diffEntriesPromise);
  const latestRevision = diff.revisions[diff.revisions.length - 1];

  return (
    <div>
      <div className="mx-16 px-1 pt-6 flex flex-row gap-4">
        <ReviewDiffMessage message={diff.message} />
        <ReviewDiffActions
          key={diff.position}
          owner={owner}
          repo={repo}
          review={review}
          position={diff.position}
          status={diff.status}
          revision={latestRevision}
        />
      </div>
      <div className="mx-16 flex flex-col gap-6 py-4">
        {entries.map((entry) => (
          <ReviewDiffFile
            key={entry.resource.path}
            diffId={diff.id}
            revisionId={latestRevision.id}
            diffFile={entry.resource}
            diffSpans={entry.spans}
          />
        ))}
      </div>
    </div>
  );
}
