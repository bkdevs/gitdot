"use client";

import type { RepositoryDiffFileResource } from "gitdot-api";
import { useCallback, useRef } from "react";
import { DiffCreated } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-created";
import { DiffSplit } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-split";
import { DiffUnified } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-unified";
import { DiffUnilateral } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-unilateral";
import { preferSplit } from "@/(main)/[owner]/[repo]/util";
import type { DiffSpans } from "@/actions";
import { cn } from "@/util";
import { useReviewContext } from "../context";
import {
  COMMENT_WIDGET_HEIGHT,
  ReviewDiffFileCommentNew,
  type ReviewDiffFileCommentNewHandle,
} from "./review-diff-file-comment-new";

const getTokenSpan = (target: EventTarget | null): HTMLElement | null => {
  if (!(target instanceof HTMLElement)) return null;
  const direct = target.closest(".diff-token");
  if (direct) return direct as HTMLElement;
  const line = target.closest(".diff-line");
  if (!line) return null;
  const tokens = line.querySelectorAll<HTMLElement>(".diff-token");
  return tokens.length ? tokens[tokens.length - 1] : null;
};

export function ReviewDiffFileBody({
  diffId,
  revisionId,
  diffFile,
  diffSpans,
  layout = "heuristic",
  className,
  onBubble,
}: {
  diffId: string;
  revisionId: string;
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
  layout?: "split" | "unified" | "heuristic";
  className?: string;
  onBubble?: (viewportTop: number | null) => void;
}) {
  const { addComment } = useReviewContext();
  const onAddComment = useCallback(
    (body: string) =>
      addComment({
        diff_id: diffId,
        revision_id: revisionId,
        body,
        file_path: diffFile.path,
      }),
    [addComment, diffId, revisionId, diffFile.path],
  );
  const containerRef = useRef<HTMLDivElement>(null);
  const startSpanRef = useRef<HTMLElement | null>(null);
  const endSpanRef = useRef<HTMLElement | null>(null);
  const allSpansRef = useRef<HTMLElement[]>([]);
  const commentRef = useRef<ReviewDiffFileCommentNewHandle | null>(null);

  const clearSelection = useCallback(() => {
    const container = containerRef.current;
    if (!container) return;

    container.querySelectorAll(".token-selected").forEach((el) => {
      el.classList.remove("token-selected");
    });
    container.classList.remove("has-selection");
    endSpanRef.current = null;
    onBubble?.(null);
  }, [onBubble]);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      clearSelection();
      containerRef.current?.classList.add("is-dragging");
      startSpanRef.current = null;
      allSpansRef.current = Array.from(
        containerRef.current?.querySelectorAll(".diff-token") ?? [],
      ) as HTMLElement[];

      const token = getTokenSpan(e.target);
      if (token) {
        e.preventDefault();
        startSpanRef.current = token;
        token.classList.add("token-selected");
        containerRef.current?.classList.add("has-selection");
      }
    },
    [clearSelection],
  );

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!(e.buttons & 1)) return;
    if (!startSpanRef.current) return;

    const token = getTokenSpan(e.target);
    if (!token) return;

    const spans = allSpansRef.current;
    const startIdx = spans.indexOf(startSpanRef.current);
    const endIdx = spans.indexOf(token);
    if (startIdx === -1 || endIdx === -1) return;

    endSpanRef.current = token;

    const [from, to] =
      startIdx <= endIdx ? [startIdx, endIdx] : [endIdx, startIdx];
    for (let i = 0; i < spans.length; i++) {
      spans[i].classList.toggle("token-selected", i >= from && i <= to);
    }
  }, []);

  const handleMouseUp = useCallback(
    (_e: React.MouseEvent) => {
      containerRef.current?.classList.remove("is-dragging");
      if (startSpanRef.current !== null) {
        const spans = allSpansRef.current;
        const startIdx = spans.indexOf(startSpanRef.current);
        const end = endSpanRef.current ?? startSpanRef.current;
        const endIdx = spans.indexOf(end);

        const isReversed = endIdx < startIdx;
        const anchorToken = isReversed
          ? spans[endIdx]
          : spans[Math.max(startIdx, endIdx)];
        const anchorLine = anchorToken.closest<HTMLElement>(".diff-line");
        if (!anchorLine)
          throw new Error("anchorToken has no .diff-line ancestor");
        const leftmostToken = anchorLine.querySelector<HTMLElement>(
          ".diff-token.token-selected",
        );
        if (!leftmostToken)
          throw new Error("anchorLine has no .diff-token.token-selected");

        const rect = leftmostToken.getBoundingClientRect();

        const pos = isReversed
          ? { x: rect.left - 16, y: rect.top - COMMENT_WIDGET_HEIGHT + 8 }
          : { x: rect.left - 16, y: rect.bottom - 8 };

        commentRef.current?.open(pos);

        const bubbleLine =
          startSpanRef.current.closest<HTMLElement>(".diff-line");
        if (bubbleLine) {
          onBubble?.(bubbleLine.getBoundingClientRect().top);
        }
      }
      startSpanRef.current = null;
      endSpanRef.current = null;
    },
    [onBubble],
  );

  const useSplit =
    diffSpans.kind === "split" &&
    (layout === "split" ||
      (layout === "heuristic" &&
        preferSplit(
          diffSpans.leftSpans,
          diffSpans.rightSpans,
          diffSpans.hunks,
        )));

  return (
    <div
      ref={containerRef}
      className={cn(
        "w-full cursor-default select-none relative",
        "[&.is-dragging_.diff-token]:cursor-default",
        "[&.has-selection_.diff-token:not(.token-selected)]:opacity-40",
        "[&.has-selection_.diff-token:not(.token-selected)]:transition-opacity",
        "[&.has-selection_.diff-token:not(.token-selected)]:duration-200",
        "[&.has-selection_.diff-token.token-selected]:opacity-100",
        "[&.has-selection_.diff-token.token-selected]:transition-opacity",
        "[&.has-selection_.diff-token.token-selected]:duration-200",
        className,
      )}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
    >
      {diffSpans.kind === "split" &&
        (useSplit ? (
          <DiffSplit
            leftSpans={diffSpans.leftSpans}
            rightSpans={diffSpans.rightSpans}
            hunks={diffSpans.hunks}
          />
        ) : (
          <DiffUnified
            leftSpans={diffSpans.leftSpans}
            rightSpans={diffSpans.rightSpans}
            hunks={diffSpans.hunks}
          />
        ))}
      {diffSpans.kind === "unilateral" && (
        <DiffUnilateral
          spans={diffSpans.spans}
          hunks={diffSpans.hunks}
          side={diffSpans.side}
        />
      )}
      {diffSpans.kind === "created" && <DiffCreated spans={diffSpans.spans} />}
      {diffSpans.kind === "deleted" && (
        <div className="text-sm font-mono px-2 text-primary/50">
          File deleted.
        </div>
      )}
      {(!diffSpans || diffSpans.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
      <ReviewDiffFileCommentNew
        ref={commentRef}
        onAddComment={onAddComment}
        onClose={clearSelection}
      />
    </div>
  );
}
