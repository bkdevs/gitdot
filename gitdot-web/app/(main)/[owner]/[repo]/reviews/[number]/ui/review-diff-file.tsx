"use client";

import type { RepositoryDiffFileResource } from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { useState } from "react";
import type { DiffData } from "@/actions";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diff,
  data,
}: {
  diff: RepositoryDiffFileResource;
  data: DiffData;
}) {
  const [open, setOpen] = useState(false);

  return (
    <>
      <div
        data-diff-file
        className="rounded-sm border border-border overflow-hidden"
      >
        <ReviewDiffFileHeader diff={diff} onClick={() => setOpen(true)} />
        <ContextMenu>
          <ContextMenuTrigger asChild>
            <div>
              <ReviewDiffFileBody data={data} />
            </div>
          </ContextMenuTrigger>
          <ContextMenuContent>
            <ContextMenuItem onSelect={() => setOpen(true)}>
              <Maximize2 />
              Expand
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </div>
      <ReviewDiffFileDialog
        diff={diff}
        data={data}
        open={open}
        setOpen={setOpen}
      />
    </>
  );
}
