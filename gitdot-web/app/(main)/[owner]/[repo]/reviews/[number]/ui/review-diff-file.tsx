"use client";

import type {
  RepositoryDiffFileResource,
  ReviewCommentResource,
} from "gitdot-api";
import { Maximize2 } from "lucide-react";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { useLayoutEffect, useMemo, useRef, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import type { DiffSpans } from "@/actions";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/ui/context-menu";
import { ReviewDiffFileBody } from "./review-diff-file-body";
import { ReviewDiffFileBubbles } from "./review-diff-file-bubbles";
import { ReviewDiffFileDialog } from "./review-diff-file-dialog";
import { ReviewDiffFileHeader } from "./review-diff-file-header";

export function ReviewDiffFile({
  diffId,
  revisionId,
  diffFile,
  diffSpans,
  diffComments,
}: {
  diffId: string;
  revisionId: string;
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
  diffComments: ReviewCommentResource[];
}) {
  const { user } = useUserContext();
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const commentParam = searchParams.get("comment");

  function handleCommentClick(comment: ReviewCommentResource) {
    const params = new URLSearchParams(searchParams.toString());
    params.set("comment", comment.id.slice(0, 8));
    router.push(`${pathname}?${params.toString()}`);
  }

  function handleCommentClose() {
    const params = new URLSearchParams(searchParams.toString());
    params.delete("comment");
    router.push(`${pathname}?${params.toString()}`);
  }

  const fileComments = useMemo(
    () => diffComments.filter((c) => c.file_path === diffFile.path),
    [diffComments, diffFile.path],
  );

  const activeComment = useMemo(
    () =>
      commentParam
        ? (fileComments.find((c) => c.id.startsWith(commentParam)) ?? null)
        : null,
    [commentParam, fileComments],
  );

  const [commentPositions, setCommentPositions] = useState<
    Array<{ top: number; comments: ReviewCommentResource[] }>
  >([]);

  useLayoutEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const groups = new Map<string, ReviewCommentResource[]>();
    for (const comment of fileComments) {
      if (comment.line_number_start === null || comment.side === null) continue;
      const key = `${comment.line_number_start}:${comment.side}`;
      const existing = groups.get(key);
      if (existing) existing.push(comment);
      else groups.set(key, [comment]);
    }

    const containerRect = container.getBoundingClientRect();
    const positions: Array<{ top: number; comments: ReviewCommentResource[] }> =
      [];

    for (const [key, comments] of groups) {
      const [lineNum, side] = key.split(":");
      const el = container.querySelector<HTMLElement>(
        `.diff-line[data-line-number="${lineNum}"][data-side="${side}"]`,
      );
      if (!el) continue;
      positions.push({
        top: el.getBoundingClientRect().top - containerRect.top,
        comments,
      });
    }

    setCommentPositions(positions);
  }, [fileComments]);

  return (
    <div ref={containerRef} className="relative">
      <div
        data-diff-file
        className="rounded-sm border border-border overflow-hidden"
      >
        <ReviewDiffFileHeader
          diffFile={diffFile}
          onClick={() => setOpen(true)}
        />
        <ContextMenu>
          <ContextMenuTrigger asChild>
            <div>
              <ReviewDiffFileBody
                diffId={diffId}
                revisionId={revisionId}
                diffFile={diffFile}
                diffSpans={diffSpans}
                fileComments={fileComments}
                activeComment={activeComment}
                onCommentClick={handleCommentClick}
                onCommentClose={handleCommentClose}
              />
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
      <ReviewDiffFileBubbles
        commentThreads={commentPositions}
        userId={user?.id}
        activeComment={activeComment}
      />
      <ReviewDiffFileDialog
        diff={diffFile}
        spans={diffSpans}
        open={open}
        setOpen={setOpen}
      />
    </div>
  );
}
