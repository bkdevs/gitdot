"use client";

import { useCallback, useLayoutEffect, useRef } from "react";
import { useReviewContext } from "../../context";
import {
  COMMENT_WIDGET_HEIGHT,
  type ReviewDiffFileCommentNewHandle,
} from "../review-diff-file-comment-new";
import type { ReviewDiffFileCommentThreadHandle } from "../review-diff-file-comment-thread";
import { readLineNumber, readSide } from "./util";

type SelectionRange = {
  lineNumberStart: number;
  lineNumberEnd: number;
  startCharacter: number;
  endCharacter: number;
  side: "old" | "new";
};

export function useCommentSelection() {
  const { activeComment, activeDiffComments, setActiveComment } =
    useReviewContext();
  const selectionRef = useRef<SelectionRange | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const allSpansRef = useRef<HTMLElement[]>([]);
  const dragStartRef = useRef<HTMLElement | null>(null);
  const dragEndRef = useRef<HTMLElement | null>(null);
  const newCommentRef = useRef<ReviewDiffFileCommentNewHandle | null>(null);
  const commentThreadRef = useRef<ReviewDiffFileCommentThreadHandle | null>(
    null,
  );

  useLayoutEffect(() => {
    const container = containerRef.current;
    if (!activeComment || !container) {
      console.log("no active comment or container");
      commentThreadRef.current?.close();
      return;
    }
    const pos = computeCommentThreadPos(activeComment.id, container);
    if (!pos) {
      console.log("no pos computed");
      commentThreadRef.current?.close();
      return;
    }
    console.log("opening at pos", pos);
    commentThreadRef.current?.open(pos);
  }, [activeComment]);

  const clearSelection = useCallback(() => {
    const container = containerRef.current;
    if (!container) return;
    container.classList.remove("has-selection");
    for (const el of container.querySelectorAll<HTMLElement>(
      ".diff-token.token-selected",
    )) {
      if (!el.dataset.commentId) el.classList.remove("token-selected");
    }
    dragEndRef.current = null;
    selectionRef.current = null;
  }, []);

  const handleClick = useCallback(
    (e: React.MouseEvent) => {
      const token = getTokenSpan(e.target);
      if (token?.dataset.commentId) {
        const isAlreadyActive = activeComment?.id === token.dataset.commentId;
        const comment = isAlreadyActive
          ? null
          : (activeDiffComments.find((c) => c.id === token.dataset.commentId) ??
            null);
        setActiveComment(comment);
      }
    },
    [activeComment, activeDiffComments, setActiveComment],
  );

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    const token = getTokenSpan(e.target);
    if (token?.dataset.commentId) {
      e.preventDefault();
      return;
    }

    containerRef.current?.classList.add("is-dragging");
    dragStartRef.current = null;
    allSpansRef.current = Array.from(
      containerRef.current?.querySelectorAll(".diff-token") ?? [],
    ) as HTMLElement[];

    if (token) {
      e.preventDefault();
      dragStartRef.current = token;
      token.classList.add("token-selected");
      containerRef.current?.classList.add("has-selection");
    }
  }, []);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!(e.buttons & 1)) return;
    if (!dragStartRef.current) return;

    const token = getTokenSpan(e.target);
    if (!token) return;

    const spans = allSpansRef.current;
    const startIdx = spans.indexOf(dragStartRef.current);
    const endIdx = spans.indexOf(token);
    if (startIdx === -1 || endIdx === -1) return;

    dragEndRef.current = token;

    const [from, to] =
      startIdx <= endIdx ? [startIdx, endIdx] : [endIdx, startIdx];
    for (let i = 0; i < spans.length; i++) {
      const isInRange = i >= from && i <= to;
      if (!spans[i].dataset.commentId) {
        spans[i].classList.toggle("token-selected", isInRange);
      }
    }
  }, []);

  const handleMouseUp = useCallback((_e: React.MouseEvent) => {
    containerRef.current?.classList.remove("is-dragging");

    if (dragStartRef.current !== null) {
      const spans = allSpansRef.current;
      const startIdx = spans.indexOf(dragStartRef.current);
      const end = dragEndRef.current ?? dragStartRef.current;
      const endIdx = spans.indexOf(end);
      const isReversed = endIdx < startIdx;

      const anchorToken = isReversed
        ? spans[endIdx]
        : spans[Math.max(startIdx, endIdx)];
      const anchorLine = anchorToken.closest<HTMLElement>(".diff-line");
      if (!anchorLine)
        throw new Error("anchorToken has no .diff-line ancestor");

      newCommentRef.current?.open(computeNewCommentPos(anchorLine, isReversed));
      selectionRef.current = computeSelectionRange(
        spans,
        startIdx,
        endIdx,
        isReversed,
      );
    }

    dragStartRef.current = null;
    dragEndRef.current = null;
  }, []);

  return {
    containerRef,
    newCommentRef,
    commentThreadRef,
    selectionRef,
    clearSelection,
    handleClick,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,
  };
}

const computeNewCommentPos = (
  anchorLine: HTMLElement,
  isReversed: boolean,
): { x: number; y: number } => {
  const leftmostToken = anchorLine.querySelector<HTMLElement>(
    ".diff-token.token-selected",
  );
  if (!leftmostToken)
    throw new Error("anchorLine has no .diff-token.token-selected");
  const rect = leftmostToken.getBoundingClientRect();
  return isReversed
    ? { x: rect.left - 16, y: rect.top - COMMENT_WIDGET_HEIGHT + 6 }
    : { x: rect.left - 16, y: rect.bottom - 8 };
};

const computeSelectionRange = (
  spans: HTMLElement[],
  startIdx: number,
  endIdx: number,
  isReversed: boolean,
): SelectionRange => {
  const firstToken = isReversed ? spans[endIdx] : spans[startIdx];
  const lastToken = isReversed ? spans[startIdx] : spans[endIdx];

  const firstLine = firstToken.closest<HTMLElement>(".diff-line");
  const lastLine = lastToken.closest<HTMLElement>(".diff-line");
  if (!firstLine || !lastLine)
    throw new Error("diff selection is missing .diff-line ancestor");

  const lineNumberStart = readLineNumber(firstLine);
  const lineNumberEnd = readLineNumber(lastLine);
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
  return {
    lineNumberStart,
    lineNumberEnd,
    startCharacter: parseInt(startCharacter, 10),
    endCharacter: parseInt(endCharacter, 10),
    side: readSide(firstLine),
  };
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

const computeCommentThreadPos = (
  commentId: string,
  container: HTMLElement,
): { x: number; y: number } | null => {
  const tokens = Array.from(
    container.querySelectorAll<HTMLElement>(
      `.diff-token[data-comment-id="${commentId}"]`,
    ),
  );
  console.log(commentId);
  if (tokens.length === 0) {
    return null;
  }

  let lastLine: HTMLElement | null = null;
  let lastLineFirstToken: HTMLElement | null = null;
  for (const token of tokens) {
    const line = token.closest<HTMLElement>(".diff-line");
    if (!line) continue;
    if (line !== lastLine) {
      lastLine = line;
      lastLineFirstToken = token;
    }
  }
  if (!lastLineFirstToken) return null;

  const rect = lastLineFirstToken.getBoundingClientRect();
  return { x: rect.left - 16, y: rect.bottom };
};
