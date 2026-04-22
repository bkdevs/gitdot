import type { ReviewCommentResource } from "gitdot-api";
import { useLayoutEffect } from "react";
import type { DiffSpans } from "@/actions";
import { readLineNumber, readSide } from "./util";

export function useHighlightComments(
  containerRef: React.RefObject<HTMLDivElement | null>,
  diffFileComments: ReviewCommentResource[],
  activeComment: ReviewCommentResource | null,
  spans: DiffSpans,
) {
  // biome-ignore lint/correctness/useExhaustiveDependencies: spans is a cache key that triggers re-run when diff content changes, resetting classes React wiped during reconciliation
  useLayoutEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const allSpans = Array.from(
      container.querySelectorAll<HTMLElement>(".diff-token"),
    );

    for (const span of allSpans) {
      span.classList.remove("token-selected");
      delete span.dataset.commentId;
    }

    for (const comment of diffFileComments) {
      if (
        comment.line_number_start == null ||
        comment.line_number_end == null ||
        comment.start_character == null ||
        comment.end_character == null
      )
        continue;

      const {
        line_number_start,
        line_number_end,
        start_character,
        end_character,
        side,
      } = comment;

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

      if (startIdx === -1 || endIdx === -1) continue;

      for (let i = startIdx; i <= endIdx; i++) {
        allSpans[i].classList.add("token-selected");
        allSpans[i].dataset.commentId = comment.id;
      }
    }
  }, [containerRef, diffFileComments, spans]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: spans is a cache key that triggers re-run when diff content changes, resetting classes React wiped during reconciliation
  useLayoutEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    for (const span of container.querySelectorAll<HTMLElement>(
      ".diff-token.token-active",
    )) {
      span.classList.remove("token-active");
    }
    if (activeComment) {
      for (const span of container.querySelectorAll<HTMLElement>(
        `.diff-token[data-comment-id="${activeComment.id}"]`,
      )) {
        span.classList.add("token-active");
      }
    }
  }, [containerRef, activeComment, spans]);
}
