"use client";

import type { DiffData } from "@/actions";
import { DiffCreated } from "../../../commits/[sha]/ui/diff-created";
import { DiffUnified } from "../../../commits/[sha]/ui/diff-unified";
import { DiffUnilateral } from "../../../commits/[sha]/ui/diff-unilateral";

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
      {data.kind === "unilateral" && (
        <DiffUnilateral
          spans={data.spans}
          hunks={data.hunks}
          side={data.side}
        />
      )}
      {data.kind === "created" && <DiffCreated spans={data.spans} />}
      {data.kind === "deleted" && (
        <div className="text-sm font-mono px-2 text-primary/50">
          File deleted.
        </div>
      )}
      {(!data || data.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
    </div>
  );
}
