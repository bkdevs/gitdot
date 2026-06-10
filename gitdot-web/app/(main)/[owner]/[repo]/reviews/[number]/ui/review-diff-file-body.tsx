"use client";

import type { ReviewCommentResource } from "gitdot-api";
import type { DiffEntry } from "gitdot-dal/client";
import { useCallback } from "react";
import { DiffCreated } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-created";
import { DiffSplit } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-split";
import { DiffUnified } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-unified";
import { DiffUnilateral } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/diff-unilateral";
import { preferSplit, splitMaxLineLength } from "@/(main)/[owner]/[repo]/util";
import { useElementWidth } from "@/hooks/use-element-width";
import { cn } from "@/util";
import { useReviewContext } from "../context";
import { useCommentSelection } from "./hooks/use-comment-selection";
import { useHighlightComments } from "./hooks/use-highlight-comments";
import { ReviewDiffFileCommentNew } from "./review-diff-file-comment-new";
import { ReviewDiffFileCommentThread } from "./review-diff-file-comment-thread";

export function ReviewDiffFileBody({
  entry,
  diffFileComments,
  layout = "heuristic",
  className,
}: {
  entry: DiffEntry;
  diffFileComments: ReviewCommentResource[];
  layout?: "split" | "unified" | "heuristic";
  className?: string;
}) {
  const { path, spans: diffSpans } = entry;
  const { addComment, activeComment, setActiveComment } = useReviewContext();
  const {
    containerRef,
    newCommentRef,
    commentThreadRef,
    selectionRef,
    clearSelection,
    handleClick,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,
  } = useCommentSelection();
  useHighlightComments(
    containerRef,
    diffFileComments,
    activeComment,
    diffSpans,
  );
  const containerWidth = useElementWidth(containerRef);

  const newComment = useCallback(
    (body: string) =>
      addComment({
        body,
        file_path: path,
        line_number_start: selectionRef.current?.lineNumberStart,
        line_number_end: selectionRef.current?.lineNumberEnd,
        start_character: selectionRef.current?.startCharacter,
        end_character: selectionRef.current?.endCharacter,
        side: selectionRef.current?.side,
      }),
    [addComment, path, selectionRef],
  );

  const useSplit =
    diffSpans.kind === "split" &&
    (layout === "split" ||
      (layout === "heuristic" &&
        preferSplit(
          diffSpans.leftSpans,
          diffSpans.rightSpans,
          diffSpans.hunks,
          splitMaxLineLength(containerWidth),
        )));

  return (
    <div
      ref={containerRef}
      className={cn(
        "w-full cursor-default select-none relative",
        // dragging
        "[&.is-dragging_.diff-token]:cursor-default",
        // selection: dim non-selected tokens, restore selected, suppress already-commented
        "[&.has-selection_.diff-token:not(.token-selected):not(.token-active)]:opacity-40",
        "[&.has-selection_.diff-token:not(.token-selected):not(.token-active)]:transition-opacity",
        "[&.has-selection_.diff-token:not(.token-selected):not(.token-active)]:duration-200",
        "[&.has-selection_.diff-token.token-selected]:opacity-100",
        "[&.has-selection_.diff-token.token-selected]:transition-opacity",
        "[&.has-selection_.diff-token.token-selected]:duration-200",
        "[&.has-selection_.diff-token.token-selected[data-comment-id]]:opacity-40",
        "[&.has-selection_.diff-token.token-selected[data-comment-id]]:bg-transparent",
        // active comment: dim lines that don't contain the active token
        "[&.has-active-comment_.diff-line:not(:has(.token-active))]:opacity-30",
        "[&.has-active-comment_.diff-line:not(:has(.token-active))]:transition-opacity",
        "[&.has-active-comment_.diff-line:not(:has(.token-active))]:duration-200",
        activeComment?.file_path === path && "has-active-comment",
        // dim this file body when another file has the active comment
        activeComment &&
          activeComment.file_path !== path &&
          "opacity-30 transition-opacity duration-200",
        className,
      )}
      onClick={handleClick}
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
        <div className="text-sm font-mono px-2 text-foreground/50">
          File deleted.
        </div>
      )}
      {(!diffSpans || diffSpans.kind === "no-change") && (
        <div className="text-sm font-mono px-2">No changes made</div>
      )}
      <ReviewDiffFileCommentNew
        ref={newCommentRef}
        newComment={newComment}
        onClose={clearSelection}
      />
      <ReviewDiffFileCommentThread
        ref={commentThreadRef}
        onClose={() => setActiveComment(null)}
      />
    </div>
  );
}
