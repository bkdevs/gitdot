"use client";

import type {
  RepositoryDiffFileResource,
  RepositoryDiffStatResource,
} from "gitdot-api";
import { useState } from "react";
import type { DiffData } from "@/actions";
import { DiffHeader } from "../../../commits/[sha]/ui/diff-header";
import { ReviewDiffFileBody } from "./review-diff-file-body";

export function ReviewDiffFile({
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
      {open && <ReviewDiffFileBody data={data} />}
    </div>
  );
}
