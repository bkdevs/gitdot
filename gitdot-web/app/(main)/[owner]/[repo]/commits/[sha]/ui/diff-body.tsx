"use client";

import type { RepositoryDiffStatResource } from "gitdot-api";
import { useState } from "react";
import type { DiffData } from "@/actions";
import { DiffHeader } from "./diff-header";
import { DiffSingle } from "./diff-single";
import { DiffSplit } from "./diff-split";

export function DiffBody({
  stat,
  diff,
}: {
  stat: RepositoryDiffStatResource;
  diff: DiffData;
}) {
  const [open, setOpen] = useState(true);

  return (
    <>
      <DiffHeader open={open} setOpen={setOpen} stat={stat} />
      {open && (
        <div className="w-full border-b border-border">
          {diff.kind === "split" && (
            <DiffSplit
              leftSpans={diff.leftSpans}
              rightSpans={diff.rightSpans}
              hunks={diff.hunks}
            />
          )}
          {diff.kind === "single" && <DiffSingle spans={diff.spans} />}
          {(!diff || diff.kind === "no-change") && (
            <div className="text-sm font-mono px-2">No changes made</div>
          )}
        </div>
      )}
    </>
  );
}
