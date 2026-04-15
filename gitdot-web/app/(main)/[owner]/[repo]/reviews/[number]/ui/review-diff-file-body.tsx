"use client";

import type { DiffData } from "@/actions";
import { DiffSingle } from "../../../commits/[sha]/ui/diff-single";
import { DiffUnified } from "../../../commits/[sha]/ui/diff-unified";

export function ReviewDiffFileBody({ data }: { data: DiffData }) {
  return (
    <div className="w-full">
      {data.kind === "split" && (
        <DiffUnified
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
