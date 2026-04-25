"use client";

import type { RepositoryDiffFileResource } from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { useMemo, useRef, useState } from "react";
import type { DiffSpans } from "@/actions";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { useReviewContext } from "../context";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diffFile,
  diffSpans,
}: {
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
}) {
  const { activeDiffComments } = useReviewContext();
  const [dialogOpen, setDialogOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const diffFileComments = useMemo(
    () => activeDiffComments.filter((c) => c.file_path === diffFile.path),
    [activeDiffComments, diffFile.path],
  );

  return (
    <div ref={containerRef} className="relative">
      <div
        data-diff-file
        className="rounded-sm border border-border overflow-hidden"
      >
        <ReviewDiffFileHeader
          diffFile={diffFile}
          onClick={() => setDialogOpen(true)}
        />
        <ContextMenu>
          <ContextMenuTrigger asChild>
            <div>
              <ReviewDiffFileBody
                diffFile={diffFile}
                diffSpans={diffSpans}
                diffFileComments={diffFileComments}
              />
            </div>
          </ContextMenuTrigger>
          <ContextMenuContent>
            <ContextMenuItem onSelect={() => setDialogOpen(true)}>
              <Maximize2 />
              Expand
            </ContextMenuItem>
          </ContextMenuContent>
        </ContextMenu>
      </div>
      <ReviewDiffFileDialog
        diff={diffFile}
        spans={diffSpans}
        open={dialogOpen}
        setOpen={setDialogOpen}
      />
    </div>
  );
}
