"use client";

import type { ReviewDiffResource } from "gitdot-api";
import { use } from "react";
import type { DiffEntry } from "@/actions";
import { ReviewDiffActions } from "./review-diff-actions";
import { ReviewDiffFile } from "./review-diff-file";
import { ReviewDiffMessage } from "./review-diff-message";

export function ReviewDiffBody({
  owner,
  repo,
  number,
  diffPromise,
  diff,
}: {
  owner: string;
  repo: string;
  number: number;
  diffPromise: Promise<DiffEntry[]>;
  diff: ReviewDiffResource;
}) {
  const entries = use(diffPromise);
  const latestRevision = diff.revisions[diff.revisions.length - 1];

  return (
    <div>
      <div className="max-w-4xl mx-auto px-1 pt-8 flex flex-row gap-4">
        <ReviewDiffMessage message={diff.message} />
        <ReviewDiffActions
          key={diff.position}
          owner={owner}
          repo={repo}
          number={number}
          position={diff.position}
          status={diff.status}
          revision={latestRevision}
        />
      </div>
      <div className="max-w-4xl mx-auto flex flex-col gap-6 py-4">
        {entries.map((entry) => (
          <ReviewDiffFile
            key={entry.diff.path}
            diff={entry.diff}
            data={entry.data}
          />
        ))}
      </div>
    </div>
  );
}
