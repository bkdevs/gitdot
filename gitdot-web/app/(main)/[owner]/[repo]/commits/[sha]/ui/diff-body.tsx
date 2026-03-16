"use client";

import { use } from "react";
import type { DiffData } from "@/actions/repository";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export function DiffBody({
  path,
  allDiffDataPromise,
}: {
  path: string;
  allDiffDataPromise: Promise<Record<string, DiffData>>;
}) {
  const allDiffData = use(allDiffDataPromise);
  const diffData = allDiffData[path];

  return (
    <div className="w-full border-b border-border">
      {diffData?.kind === "split" && (
        <DiffSplit
          leftSpans={diffData.leftSpans}
          rightSpans={diffData.rightSpans}
          hunks={diffData.hunks}
        />
      )}
      {diffData?.kind === "single" && <DiffSingle spans={diffData.spans} />}
      {(!diffData || diffData.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
    </div>
  );
}
