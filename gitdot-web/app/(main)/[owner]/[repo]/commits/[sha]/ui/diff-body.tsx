"use client";

import type {
  RepositoryDiffFileResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import { useState } from "react";
import type { DiffData } from "@/actions";
import { DiffHeader } from "./diff-header";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export function DiffBody({
  diff,
  data,
}: {
  diff: RepositoryDiffStatResource | RepositoryDiffFileResource;
  data: DiffData;
}) {
  const [open, setOpen] = useState(true);

  return (
    <>
      <DiffHeader open={open} setOpen={setOpen} diff={diff} />
      {open && (
        <div className="w-full border-b border-border">
          {data.kind === "split" && (
            <DiffSplit
              leftSpans={data.leftSpans}
              rightSpans={data.rightSpans}
              hunks={data.hunks}
            />
          )}
          {data.kind === "single" && <DiffSingle spans={data.spans} />}
          {(!data || data.kind === "no-change") && (
            <div className="text-sm font-mono px-2">No changes made</div>
          )}
        </div>
      )}
    </>
  );
}
