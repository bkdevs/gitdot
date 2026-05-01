"use client";

import type {
  RepositoryDiffFileResource,
  ReviewCommentResource,
} from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { useLayoutEffect, useMemo, useRef, useState } from "react";
import type { DiffSpans } from "@/actions";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { useReviewContext } from "../context";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileBubbles } from "./review-diff-file-bubbles";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diffFile,
  diffSpans,
}: {
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
}) {
  const { activeDiffComments, activeDiffCommentThreads } = useReviewContext();
  const [dialogOpen, setDialogOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const diffFileComments = useMemo(
    () => activeDiffComments.filter((c) => c.file_path === diffFile.path),
    [activeDiffComments, diffFile.path],
  );

  const [bubblePositionsLeft, setBubblePositionsLeft] = useState<
    Array<{
      top: number;
      parentComment: ReviewCommentResource;
      commentCount: number;
    }>
  >([]);
  const [bubblePositionsRight, setBubblePositionsRight] = useState<
    Array<{
      top: number;
      parentComment: ReviewCommentResource;
      commentCount: number;
    }>
  >([]);

  useLayoutEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const containerRect = container.getBoundingClientRect();
    const left: Array<{
      top: number;
      parentComment: ReviewCommentResource;
      commentCount: number;
    }> = [];
    const right: Array<{
      top: number;
      parentComment: ReviewCommentResource;
      commentCount: number;
    }> = [];

    const fileThreads = activeDiffCommentThreads.filter(
      (t) => t[0].file_path === diffFile.path,
    );

    for (const thread of fileThreads) {
      const root = thread[0];
      if (root.line_number_start === null || root.side === null) continue;
      const el = container.querySelector<HTMLElement>(
        `.diff-line[data-line-number="${root.line_number_start}"][data-side="${root.side}"]`,
      );
      if (!el) continue;
      const position = {
        top: el.getBoundingClientRect().top - containerRect.top,
        parentComment: root,
        commentCount: thread.length,
      };
      if (root.side === "old") left.push(position);
      else right.push(position);
    }

    setBubblePositionsLeft(left);
    setBubblePositionsRight(right);
  }, [activeDiffCommentThreads, diffFile.path]);

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
      <ReviewDiffFileBubbles side="old" bubblePositions={bubblePositionsLeft} />
      <ReviewDiffFileBubbles
        side="new"
        bubblePositions={bubblePositionsRight}
      />
      <ReviewDiffFileDialog
        diff={diffFile}
        spans={diffSpans}
        open={dialogOpen}
        setOpen={setDialogOpen}
      />
    </div>
  );
}
