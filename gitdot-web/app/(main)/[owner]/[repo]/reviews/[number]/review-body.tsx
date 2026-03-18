"use client";

import { use } from "react";
import { DiffBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-body";
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
          <DiffBody diff={diff} data={data} />
        </DiffFileProvider>
      ))}
    </div>
  );
}
