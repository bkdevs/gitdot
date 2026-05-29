import type { DiffHunk } from "gitdot-dal/client";
import type { Element } from "hast";

// ============================================================================
// unified vs split diff view heuristics
// ============================================================================

const SPLIT_MAX_LINE_LENGTH = 80;
const SPLIT_MIN_MATCH_RATIO = 0.25;
const UNIFIED_MAX_PAIRS_COUNT = 50;

export function preferSplit(
  leftSpans: Element[],
  rightSpans: Element[],
  hunks: DiffHunk[],
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

  return (
    maxLen <= SPLIT_MAX_LINE_LENGTH && matched / total >= SPLIT_MIN_MATCH_RATIO
  );
}

function spanTextLength(span: Element): number {
  let len = 0;
  for (const child of span.children) {
    if (child.type === "text") len += child.value.length;
    else if (child.type === "element") len += spanTextLength(child);
  }
  return len;
}
