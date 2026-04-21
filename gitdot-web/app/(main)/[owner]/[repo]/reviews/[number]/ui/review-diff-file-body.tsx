"use client";

import type {
  RepositoryDiffFileResource,
  ReviewCommentResource,
} from "gitdot-api";
import { useCallback, useEffect, useRef } from "react";
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

export function ReviewDiffFileBody({
  diffId,
  revisionId,
  diffFile,
  diffSpans,
  activeComment,
  layout = "heuristic",
  className,
}: {
  diffId: string;
  revisionId: string;
  diffFile: RepositoryDiffFileResource;
  diffSpans: DiffSpans;
  activeComment?: ReviewCommentResource | null;
  layout?: "split" | "unified" | "heuristic";
  className?: string;
}) {
  const { addComment } = useReviewContext();
  const selectionRef = useRef<{
    lineNumberStart: number;
    lineNumberEnd: number;
    startCharacter: number;
    endCharacter: number;
    side: "old" | "new";
  } | null>(null);
  const onAddComment = useCallback(
    (body: string) =>
      addComment({
        diff_id: diffId,
        revision_id: revisionId,
        body,
        file_path: diffFile.path,
        line_number_start: selectionRef.current?.lineNumberStart,
        line_number_end: selectionRef.current?.lineNumberEnd,
        start_character: selectionRef.current?.startCharacter,
        end_character: selectionRef.current?.endCharacter,
        side: selectionRef.current?.side,
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
    selectionRef.current = null;
  }, []);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    container.querySelectorAll(".token-selected").forEach((el) => {
      el.classList.remove("token-selected");
    });
    container.classList.remove("has-selection");

    if (
      !activeComment ||
      activeComment.line_number_start == null ||
      activeComment.line_number_end == null ||
      activeComment.start_character == null ||
      activeComment.end_character == null
    )
      return;

    const {
      line_number_start,
      line_number_end,
      start_character,
      end_character,
      side,
    } = activeComment;

    const allSpans = Array.from(
      container.querySelectorAll<HTMLElement>(".diff-token"),
    );
    let startIdx = -1;
    let endIdx = -1;
    for (let i = 0; i < allSpans.length; i++) {
      const span = allSpans[i];
      const line = span.closest<HTMLElement>(".diff-line");
      if (!line) continue;
      if (side && readSide(line) !== side) continue;
      const lineNum = readLineNumber(line);
      const charStart = parseInt(span.dataset.charStart ?? "-1", 10);
      const charEnd = parseInt(span.dataset.charEnd ?? "-1", 10);
      if (
        startIdx === -1 &&
        lineNum === line_number_start &&
        charStart === start_character
      ) {
        startIdx = i;
      }
      if (lineNum === line_number_end && charEnd === end_character) {
        endIdx = i;
      }
    }

    if (startIdx === -1 || endIdx === -1) return;

    for (let i = startIdx; i <= endIdx; i++) {
      allSpans[i].classList.add("token-selected");
    }
    container.classList.add("has-selection");
  }, [activeComment]);

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

  const handleMouseUp = useCallback((_e: React.MouseEvent) => {
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
        ? { x: rect.left - 16, y: rect.top - COMMENT_WIDGET_HEIGHT + 6 }
        : { x: rect.left - 16, y: rect.bottom - 8 };

      commentRef.current?.open(pos);

      const firstToken = isReversed ? end : startSpanRef.current;
      const lastToken = isReversed ? startSpanRef.current : end;
      const firstLine = firstToken.closest<HTMLElement>(".diff-line");
      const lastLine = lastToken.closest<HTMLElement>(".diff-line");
      const lineNumberStart = firstLine ? readLineNumber(firstLine) : undefined;
      const lineNumberEnd = lastLine ? readLineNumber(lastLine) : undefined;
      const startCharacter = firstToken.dataset.charStart;
      const endCharacter = lastToken.dataset.charEnd;

      if (
        lineNumberStart === undefined ||
        lineNumberEnd === undefined ||
        startCharacter === undefined ||
        endCharacter === undefined
      ) {
        throw new Error("diff selection is missing required line/char data");
      }
      selectionRef.current = {
        lineNumberStart,
        lineNumberEnd,
        startCharacter: parseInt(startCharacter, 10),
        endCharacter: parseInt(endCharacter, 10),
        // biome-ignore lint/style/noNonNullAssertion: firstLine is non-null when lineNumberStart is defined
        side: readSide(firstLine!),
      };
    }
    startSpanRef.current = null;
    endSpanRef.current = null;
  }, []);

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

const readLineNumber = (line: HTMLElement): number | undefined => {
  if (line.dataset.lineNumber) return parseInt(line.dataset.lineNumber, 10);
  const lineType = line.dataset.lineType;
  if (lineType === "removed" && line.dataset.leftLineNumber)
    return parseInt(line.dataset.leftLineNumber, 10);
  if (line.dataset.rightLineNumber)
    return parseInt(line.dataset.rightLineNumber, 10);
  if (line.dataset.leftLineNumber)
    return parseInt(line.dataset.leftLineNumber, 10);
  return undefined;
};

const readSide = (line: HTMLElement): "old" | "new" => {
  const raw = line.dataset.side;
  if (raw === "old") return "old";
  if (raw === "new") return "new";
  return line.dataset.lineType === "removed" ? "old" : "new";
};

const getTokenSpan = (target: EventTarget | null): HTMLElement | null => {
  if (!(target instanceof HTMLElement)) return null;
  const direct = target.closest(".diff-token");
  if (direct) return direct as HTMLElement;
  const line = target.closest(".diff-line");
  if (!line) return null;
  const tokens = line.querySelectorAll<HTMLElement>(".diff-token");
  return tokens.length ? tokens[tokens.length - 1] : null;
};
