"use client";

import type { DiffEntry } from "@/actions";
import type { ReviewDiffResource } from "gitdot-api";
import { use } from "react";
import { ReviewDiffFile } from "./review-diff-file";
import { ReviewDiffMessage } from "./review-diff-message";

export function ReviewDiffBody({
  diffPromise,
  diff,
}: {
  diffPromise: Promise<DiffEntry[]>;
  diff: ReviewDiffResource;
}) {
  const entries = use(diffPromise);

  return (
    <div>
      <ReviewDiffMessage message={diff.message} />
      <div className="max-w-3xl mx-auto flex flex-col gap-6 py-4">
        {entries.map((entry) => (
          <ReviewDiffFile key={entry.diff.path} diff={entry.diff} data={entry.data} />
        ))}
      </div>
    </div>
  );
}
