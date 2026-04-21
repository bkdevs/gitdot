"use client";

import type {
  RepositoryDiffFileResource,
  ReviewCommentResource,
} from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
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
import {
  ReviewDiffFileCommentThread,
  type ReviewDiffFileCommentThreadHandle,
} from "./review-diff-file-comment-thread";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diffFile,
  diffSpans,
}: {
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
}) {
  const { activeDiffComments, activeComment, setActiveComment } =
    useReviewContext();
  const [dialogOpen, setDialogOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const threadRef = useRef<ReviewDiffFileCommentThreadHandle | null>(null);
  const diffFileComments = useMemo(
    () => activeDiffComments.filter((c) => c.file_path === diffFile.path),
    [activeDiffComments, diffFile.path],
  );

  useEffect(() => {
    if (!activeComment || !containerRef.current || !threadRef.current) {
      threadRef.current?.close();
      return;
    }
    const token = containerRef.current.querySelector<HTMLElement>(
      `.diff-token[data-comment-id="${activeComment.id}"]`,
    );
    if (!token) {
      threadRef.current.close();
      return;
    }
    const threadComments = diffFileComments.filter(
      (c) =>
        c.line_number_start === activeComment.line_number_start &&
        c.side === activeComment.side,
    );
    const rect = token.getBoundingClientRect();
    threadRef.current.open(
      { x: rect.left - 16, y: rect.bottom - 8 },
      threadComments,
    );
  }, [activeComment, diffFileComments]);

  const [bubblePositionsLeft, setBubblePositionsLeft] = useState<
    Array<{ top: number; comments: ReviewCommentResource[] }>
  >([]);
  const [bubblePositionsRight, setBubblePositionsRight] = useState<
    Array<{ top: number; comments: ReviewCommentResource[] }>
  >([]);

  useLayoutEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const groups = new Map<string, ReviewCommentResource[]>();
    for (const comment of diffFileComments) {
      if (comment.line_number_start === null || comment.side === null) continue;
      const key = `${comment.line_number_start}:${comment.side}`;
      const existing = groups.get(key);
      if (existing) existing.push(comment);
      else groups.set(key, [comment]);
    }

    const containerRect = container.getBoundingClientRect();
    const left: Array<{ top: number; comments: ReviewCommentResource[] }> = [];
    const right: Array<{ top: number; comments: ReviewCommentResource[] }> = [];

    for (const [key, comments] of groups) {
      const [lineNum, side] = key.split(":");
      const el = container.querySelector<HTMLElement>(
        `.diff-line[data-line-number="${lineNum}"][data-side="${side}"]`,
      );
      if (!el) continue;
      const position = {
        top: el.getBoundingClientRect().top - containerRect.top,
        comments,
      };
      if (side === "old") left.push(position);
      else right.push(position);
    }

    setBubblePositionsLeft(left);
    setBubblePositionsRight(right);
  }, [diffFileComments]);

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
      <ReviewDiffFileCommentThread
        ref={threadRef}
        onClose={() => setActiveComment(null)}
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
