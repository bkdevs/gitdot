"use client";

import { useState } from "react";
import type { DiffEntry } from "@/actions";
import { DiffBody } from "./diff-body";
import { DiffHeader } from "./diff-header";

export function DiffFile({ entry }: { entry: DiffEntry }) {
  const [open, setOpen] = useState(true);

  return (
    <div data-diff-file>
      <DiffHeader
        open={open}
        setOpen={setOpen}
        path={entry.resource.path}
        linesAdded={entry.resource.lines_added}
        linesRemoved={entry.resource.lines_removed}
      />
      {open && (
        <DiffBody spans={entry.spans} className="border-b border-border" />
      )}
    </div>
  );
}
