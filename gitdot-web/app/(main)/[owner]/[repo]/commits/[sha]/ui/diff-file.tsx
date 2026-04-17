"use client";

import type {
  RepositoryDiffFileResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import { useState } from "react";
import type { DiffData } from "@/actions";
import { DiffBody } from "./diff-body";
import { DiffHeader } from "./diff-header";

export function DiffFile({
  diff,
  data,
}: {
  diff: RepositoryDiffStatResource | RepositoryDiffFileResource;
  data: DiffData;
}) {
  const [open, setOpen] = useState(true);

  return (
    <div data-diff-file>
      <DiffHeader open={open} setOpen={setOpen} diff={diff} />
      {open && <DiffBody data={data} className="border-b border-border" />}
    </div>
  );
}
