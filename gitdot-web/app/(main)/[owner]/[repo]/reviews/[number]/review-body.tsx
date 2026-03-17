"use client";

import { use } from "react";
import { DiffBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-body";
import type { DiffEntry } from "@/actions";

export function ReviewBody({
  diffEntries,
}: {
  diffEntries: Promise<DiffEntry[]>;
}) {
  const entries = use(diffEntries);

  return (
    <div className="flex flex-col">
      {entries.map(({ diff, data }) => (
        <DiffBody key={diff.path} diff={diff} data={data} />
      ))}
    </div>
  );
}
