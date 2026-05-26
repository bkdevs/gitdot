import { structuredPatch } from "diff";
import type { Element } from "hast";

export type LinePair = [number | null, number | null];
export const CONTEXT_LINES = 4;

export type DiffLine = { line_number: number };
export type DiffPair = { lhs?: DiffLine; rhs?: DiffLine };
export type DiffHunk = DiffPair[];

/**
 * builds DiffHunk[] from raw file contents using jsdiff's structuredPatch
 *
 * context: CONTEXT_LINES bundles adjacent changes within CONTEXT_LINES * 2 of
 * each other into one hunk, and includes up to CONTEXT_LINES of boundary
 * context on each side of the hunk (clamped by file boundaries).
 *
 * we preserve every line in original order so intra-hunk context lines become
 * both-sided anchor pairs that pairLines can use as alignment points. removed
 * lines are emitted as lhs-only, added as rhs-only.
 */
export function diffFiles(
  leftContent: string,
  rightContent: string,
): DiffHunk[] {
  const patch = structuredPatch(
    "left",
    "right",
    leftContent,
    rightContent,
    undefined,
    undefined,
    { context: CONTEXT_LINES },
  );

  const result: DiffHunk[] = [];
  for (const hunk of patch.hunks) {
    // structuredPatch returns 1-indexed line numbers (standard unified diff), but we 0-index elsewhere
    let oldLine = hunk.oldStart - 1;
    let newLine = hunk.newStart - 1;
    const pairs: DiffPair[] = [];

    for (const raw of hunk.lines) {
      const marker = raw[0];
      if (marker === "-") {
        pairs.push({ lhs: { line_number: oldLine } });
        oldLine++;
      } else if (marker === "+") {
        pairs.push({ rhs: { line_number: newLine } });
        newLine++;
      } else if (marker === " ") {
        pairs.push({
          lhs: { line_number: oldLine },
          rhs: { line_number: newLine },
        });
        oldLine++;
        newLine++;
      }
    }

    result.push(pairs);
  }

  return result;
}

/**
 * converts a DiffHunk to LinePair[] for split-view rendering.
 *
 * walks the hunk in order. anchors (lines with both lhs and rhs) flush any
 * pending one-sided lines and then emit as-is. consecutive removed/added lines
 * are zipped side-by-side on flush; whichever side is longer gets sentinels on
 * the other side. as a result, the left and right columns of the returned
 * array always have equal row count.
 *
 * example:
 *   in:  [-A, -B, +X, ctx_l/ctx_r, -C, +Y, +Z]
 *   out: [[A, X], [B, null], [ctx_l, ctx_r], [C, Y], [null, Z]]
 */
export function pairLines(hunk: DiffHunk): LinePair[] {
  const result: LinePair[] = [];
  let pendingLhs: number[] = [];
  let pendingRhs: number[] = [];

  const flush = () => {
    const n = Math.max(pendingLhs.length, pendingRhs.length);
    for (let i = 0; i < n; i++) {
      result.push([
        i < pendingLhs.length ? pendingLhs[i] : null,
        i < pendingRhs.length ? pendingRhs[i] : null,
      ]);
    }
    pendingLhs = [];
    pendingRhs = [];
  };

  for (const pair of hunk) {
    if (pair.lhs && pair.rhs) {
      flush();
      result.push([pair.lhs.line_number, pair.rhs.line_number]);
    } else if (pair.lhs) {
      pendingLhs.push(pair.lhs.line_number);
    } else if (pair.rhs) {
      pendingRhs.push(pair.rhs.line_number);
    }
  }
  flush();

  return result;
}

// ============================================================================
// expandLines
// ============================================================================

/**
 * boundary context is now produced by structuredPatch and preserved through
 * diffFiles as anchor pairs, so hunks with at least one anchor need no further
 * expansion. the only case we still extrapolate is a fully one-sided hunk
 * (e.g., the change spans the file, leaving no matching lines for context).
 */
export function expandLines(
  pairs: LinePair[],
  leftMax: number,
  rightMax: number,
): LinePair[] {
  if (pairs.length === 0) return pairs;

  // any anchor means boundary context already came from structuredPatch
  for (const pair of pairs) {
    if (pair[0] !== null && pair[1] !== null) return pairs;
  }

  const offset = pairs.length;
  const first = pairs[0][0] ?? (pairs[0][1] as number);

  // expand lines before
  const context: LinePair[] = [];
  const startLine = Math.max(0, first - CONTEXT_LINES);
  for (let j = startLine; j < first; j++) {
    if (j >= leftMax || j >= rightMax) continue;
    context.push([j, j]);
  }
  pairs.unshift(...context);

  // expand lines after
  // biome-ignore lint/style/noNonNullAssertion: pairs is non-empty here (early return above)
  const [lastLeft, lastRight] = pairs.at(-1)!;
  const lastValue = lastLeft !== null ? lastLeft : (lastRight as number);

  const effectiveOffset = lastLeft !== null ? offset : -offset;
  for (let j = 1; j <= CONTEXT_LINES; j++) {
    const currentBase = lastValue + j;
    const left =
      lastLeft !== null ? currentBase : currentBase + effectiveOffset;
    const right =
      lastLeft !== null ? currentBase - effectiveOffset : currentBase;

    if (left >= leftMax || right >= rightMax) break;
    pairs.push([left, right]);
  }

  return pairs;
}

// ============================================================================
// changed-line sets
// ============================================================================

/**
 * collects the set of changed line numbers per side
 *
 * used by renderSpans to tag added/removed lines for whole-line background
 * coloring. context (both-sided anchor) pairs are not changes and are excluded.
 */
export function createChangeMaps(hunks: DiffHunk[]): {
  leftLines: Set<number>;
  rightLines: Set<number>;
} {
  const leftLines = new Set<number>();
  const rightLines = new Set<number>();
  for (const hunk of hunks) {
    for (const line of hunk) {
      if (line.lhs && !line.rhs) leftLines.add(line.lhs.line_number);
      if (line.rhs && !line.lhs) rightLines.add(line.rhs.line_number);
    }
  }
  return { leftLines, rightLines };
}

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
    for (const pair of hunk) {
      if (pair.lhs) {
        const span = leftSpans[pair.lhs.line_number];
        if (span) maxLen = Math.max(maxLen, spanTextLength(span));
      }
      if (pair.rhs) {
        const span = rightSpans[pair.rhs.line_number];
        if (span) maxLen = Math.max(maxLen, spanTextLength(span));
      }
      
      // TODO: with context anchors now in every hunk, this match ratio is
      // inflated; count only non-anchor pairs against `total` if re-enabled.
      if (pair.lhs && pair.rhs) matched++;
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
