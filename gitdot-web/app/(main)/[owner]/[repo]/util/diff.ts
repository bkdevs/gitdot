import type { DiffHunk } from "gitdot-dal/client";
import type { Element } from "hast";

// ============================================================================
// unified vs split diff view heuristics
// ============================================================================

const SPLIT_MIN_MATCH_RATIO = 0.25;
const UNIFIED_MAX_PAIRS_COUNT = 50;

// Used when the diff container hasn't been measured yet (e.g. first render).
// Roughly the width of a comfortable desktop split column.
const DEFAULT_SPLIT_MAX_LINE_LENGTH = 80;

// Split view lays the diff out as two equal-width columns of `text-sm`
// monospace text, each preceded by a line-number gutter. These constants let
// us estimate how many characters actually fit in one column at a given width.
const SPLIT_COLUMN_CHAR_WIDTH_PX = 8.4; // text-sm (14px) monospace ≈ 0.6em
const SPLIT_COLUMN_GUTTER_PX = 36; // line-number gutter (w-7 + paddings)
const SPLIT_MIN_LINE_LENGTH = 40; // floor so narrow viewports still allow split

/**
 * Derive the max line length that still fits comfortably in a split column,
 * given the diff container's rendered width. In split view the container is
 * divided into two equal columns, so each line gets ~half the width minus the
 * line-number gutter. Falls back to a sensible default until measured.
 */
export function splitMaxLineLength(containerWidth: number): number {
  if (containerWidth <= 0) return DEFAULT_SPLIT_MAX_LINE_LENGTH;

  const columnTextWidth = containerWidth / 2 - SPLIT_COLUMN_GUTTER_PX;
  const fittingChars = Math.floor(columnTextWidth / SPLIT_COLUMN_CHAR_WIDTH_PX);
  return Math.max(SPLIT_MIN_LINE_LENGTH, fittingChars);
}

export function preferSplit(
  leftSpans: Element[],
  rightSpans: Element[],
  hunks: DiffHunk[],
  maxLineLength: number = DEFAULT_SPLIT_MAX_LINE_LENGTH,
): boolean {
  let maxLen = 0;
  let matched = 0;
  let total = 0;

  for (const hunk of hunks) {
    for (const [L, R] of hunk.pairs) {
      if (L !== null) {
        const span = leftSpans[L];
        if (span) maxLen = Math.max(maxLen, spanTextLength(span));
      }
      if (R !== null) {
        const span = rightSpans[R];
        if (span) maxLen = Math.max(maxLen, spanTextLength(span));
      }

      // TODO: with context anchors now in every hunk, this match ratio is
      // inflated; count only non-anchor pairs against `total` if re-enabled.
      if (L !== null && R !== null) matched++;
      total++;
    }
  }

  if (total === 0) return false;
  if (total > UNIFIED_MAX_PAIRS_COUNT) return true;

  return maxLen <= maxLineLength && matched / total >= SPLIT_MIN_MATCH_RATIO;
}

function spanTextLength(span: Element): number {
  let len = 0;
  for (const child of span.children) {
    if (child.type === "text") len += child.value.length;
    else if (child.type === "element") len += spanTextLength(child);
  }
  return len;
}
