"use client";

import type { ReviewDiffResource } from "gitdot-api";
import { use } from "react";
import type { DiffEntry } from "@/actions";
import { ReviewDiffActions } from "./review-diff-actions";
import { ReviewDiffFile } from "./review-diff-file";
import { ReviewDiffMessage } from "./review-diff-message";
import { ReviewDiffMetadata } from "./review-diff-metadata";

export function ReviewDiffCode({
  diffEntriesPromise,
  diff,
}: {
  diffEntriesPromise: Promise<DiffEntry[]>;
  diff: ReviewDiffResource;
}) {
  const entries = use(diffEntriesPromise);
  const latestRevision = diff.revisions[diff.revisions.length - 1];
  return (
    <div>
      <div className="mx-16 px-1 pt-6 flex flex-row gap-4">
        <ReviewDiffMessage message={diff.message} />
        <div className="shrink-0 flex flex-col justify-between items-end self-stretch gap-4">
          <ReviewDiffMetadata revision={latestRevision} />
          <ReviewDiffActions
            key={diff.position}
            position={diff.position}
            status={diff.status}
            revision={latestRevision}
          />
        </div>
      </div>
      <div className="mx-16 flex flex-col gap-6 pt-8 pb-4">
        {entries.map((entry) => (
          <ReviewDiffFile
            key={entry.resource.path}
            diffFile={entry.resource}
            diffSpans={entry.spans}
          />
        ))}
      </div>
    </div>
  );
}
