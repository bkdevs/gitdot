"use client";

import type { DiffData } from "@/actions";
import { DiffSingle } from "../../../commits/[sha]/ui/diff-single";
import { DiffSplit } from "../../../commits/[sha]/ui/diff-split";

export function ReviewDiffFileBody({ data }: { data: DiffData }) {
  return (
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
  );
}
