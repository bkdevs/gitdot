"use client";

import { use } from "react";
import { DiffFile } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-file";
import type { DiffEntry } from "@/actions";
import { DiffFileProvider } from "./review-comment-context";

export function ReviewBody({
  diffEntries,
}: {
  diffEntries: Promise<DiffEntry[]> | DiffEntry[];
}) {
  const entries =
    diffEntries instanceof Promise ? use(diffEntries) : diffEntries;

  return (
    <div className="flex flex-col">
      {entries.map(({ diff, data }) => (
        <DiffFileProvider key={diff.path} filePath={diff.path}>
          <DiffFile diff={diff} data={data} />
        </DiffFileProvider>
      ))}
    </div>
  );
}
