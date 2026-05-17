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
 * we use context: CONTEXT_LINES so jsdiff already bundles adjacent changes
 * within CONTEXT_LINES * 2 of each other into one hunk
 *
 * within a hunk we collect removed and added lines in order, then zip them:
 *  - first min(removed, added) lines pair lhs <-> rhs
 *  - remainder is one-sided
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
    const removed: DiffLine[] = [];
    const added: DiffLine[] = [];

    for (const raw of hunk.lines) {
      const marker = raw[0];
      if (marker === "-") {
        removed.push({ line_number: oldLine });
        oldLine++;
      } else if (marker === "+") {
        added.push({ line_number: newLine });
        newLine++;
      } else if (marker === " ") {
        oldLine++;
        newLine++;
      }
    }

    const max = Math.max(removed.length, added.length);
    if (max === 0) continue;

    const pairs: DiffPair[] = [];
    for (let i = 0; i < max; i++) {
      pairs.push({
        lhs: i < removed.length ? removed[i] : undefined,
        rhs: i < added.length ? added[i] : undefined,
      });
    }
    result.push(pairs);
  }

  return result;
}

/**
 * a tad complicated and heuristic function.
 *
 * a hunk is a sequence of paired lines, where each pair is one of:
 * - added [rhs only]
 * - removed [lhs only]
 * - modified [lhs + rhs, paired by jsdiff via order within a hunk]
 *
 * the input may contain pairs whose left/right indices are not contiguous (e.g.
 * [15, 20]). to render a properly aligned split view we need to pad sentinels
 * between anchors so each side increments by one per row.
 *
 * output:
 *  - LinePair[], a list of line numbers that indicate which left line maps to
 *    what right line (and what lines should be sentinelled)
 *
 * example output:
 *  - [1, 2] # showing offset at beginning of file
 *  - [2, 3]
 *  - [null, 4] # indicating added on rhs
 *  - [3, 5]
 *  - [4, null] # indicating removal on lhs
 *
 * invariants:
 * - each side contains the full range of indices from min, max in the hunk (inclusive)
 * - each side is monotonically increasing, excluding null sentinels
 * the list size may exceed the raw hunk size when padding is added.
 */
export function pairLines(hunk: DiffHunk): LinePair[] {
  const hunkPairs: LinePair[] = [];

  // first add all paired lines (those that are matched) and use those as anchors to generate the full alignment
  for (const line of hunk) {
    if (line.lhs && line.rhs) {
      hunkPairs.push([line.lhs.line_number, line.rhs.line_number]);
    }
  }

  // then insert lhs only lines by sorted order of the left index
  for (const line of hunk) {
    if (line.lhs && !line.rhs) {
      insertLhsInOrder(hunkPairs, line.lhs.line_number);
    }
  }

  // then do the same for rhs
  for (const line of hunk) {
    if (!line.lhs && line.rhs) {
      insertRhsInOrder(hunkPairs, line.rhs.line_number);
    }
  }

  // find anchor indices (lines where both sides are non-null)
  const anchorIndices = hunkPairs
    .map((p, i) => (p[0] !== null && p[1] !== null ? i : -1))
    .filter((i) => i !== -1);

  // if no anchors, handle all one-sided case
  if (anchorIndices.length === 0) {
    return fillGapsOneSided(hunkPairs);
  }

  const result: LinePair[] = [];

  // process each range between consecutive anchors
  for (let i = 0; i < anchorIndices.length; i++) {
    const startIdx = anchorIndices[i];
    const startAnchor = hunkPairs[startIdx];
    result.push(startAnchor);

    if (i < anchorIndices.length - 1) {
      const endIdx = anchorIndices[i + 1];
      const endAnchor = hunkPairs[endIdx];
      const entriesBetween = hunkPairs.slice(startIdx + 1, endIdx);
      result.push(...fillGapsInRange(startAnchor, endAnchor, entriesBetween));
    }
  }

  return result;
}

function insertLhsInOrder(pairs: LinePair[], lhs: number): void {
  let i = 0;
  while (i < pairs.length) {
    const currentLeft = pairs[i][0];
    if (currentLeft !== null && currentLeft >= lhs) break;
    i++;
  }
  pairs.splice(i, 0, [lhs, null]);
}

function insertRhsInOrder(pairs: LinePair[], rhs: number): void {
  let i = 0;
  while (i < pairs.length) {
    const currentRight = pairs[i][1];
    if (currentRight !== null && currentRight >= rhs) break;
    i++;
  }
  pairs.splice(i, 0, [null, rhs]);
}

/**
 * Fill gaps in one-sided entries (all lhs-only or all rhs-only)
 */
function fillGapsOneSided(pairs: LinePair[]): LinePair[] {
  if (pairs.length === 0) return [];

  const isLhsOnly = pairs[0][0] !== null;
  const result: LinePair[] = [];

  for (const pair of pairs) {
    // biome-ignore lint/style/noNonNullAssertion: isLhsOnly guarantees the corresponding side is non-null
    const value = isLhsOnly ? pair[0]! : pair[1]!;
    if (result.length > 0) {
      const last = result[result.length - 1];
      const lastValue = (isLhsOnly ? last[0] : last[1]) as number;
      for (let v = lastValue + 1; v < value; v++) {
        result.push(isLhsOnly ? [v, null] : [null, v]);
      }
    }
    result.push(pair);
  }
  return result;
}

/**
 * Fill entries between two anchors, grouping sentinels together at the start
 *
 * TODO: add test cases specifically for this, this is where we can do advanced / clever things
 * for heuristic matching and make things look "good"
 */
function fillGapsInRange(
  startAnchor: LinePair,
  endAnchor: LinePair,
  entriesBetween: LinePair[],
): LinePair[] {
  // biome-ignore lint/style/noNonNullAssertion: anchor pairs always have both sides non-null
  const lhsGap = endAnchor[0]! - startAnchor[0]! - 1;
  // biome-ignore lint/style/noNonNullAssertion: anchor pairs always have both sides non-null
  const rhsGap = endAnchor[1]! - startAnchor[1]! - 1;

  const existingLhsNulls = entriesBetween.filter((p) => p[0] === null).length;
  const existingRhsNulls = entriesBetween.filter((p) => p[1] === null).length;

  // diff > 0 means we need more lhs nulls, diff < 0 means we need more rhs nulls
  const diff = rhsGap - lhsGap - existingLhsNulls + existingRhsNulls;

  const result: LinePair[] = [];
  // biome-ignore lint/style/noNonNullAssertion: anchor pairs always have both sides non-null
  let leftPos = startAnchor[0]! + 1;
  // biome-ignore lint/style/noNonNullAssertion: anchor pairs always have both sides non-null
  let rightPos = startAnchor[1]! + 1;

  // Heuristic: place all new sentinels at the beginning, grouped together
  if (diff > 0) {
    for (let i = 0; i < diff; i++) {
      result.push([null, rightPos++]);
    }
  } else if (diff < 0) {
    for (let i = 0; i < -diff; i++) {
      result.push([leftPos++, null]);
    }
  }

  // Add existing one-sided entries
  for (const entry of entriesBetween) {
    if (entry[0] === null) {
      result.push([null, rightPos++]);
    } else if (entry[1] === null) {
      result.push([leftPos++, null]);
    }
  }

  // Fill remaining paired lines to reach endAnchor
  // biome-ignore lint/style/noNonNullAssertion: anchor pairs always have both sides non-null
  while (leftPos < endAnchor[0]! || rightPos < endAnchor[1]!) {
    result.push([leftPos++, rightPos++]);
  }

  return result;
}

// ============================================================================
// expandLines
// ============================================================================

/**
 * once we have paired lines, we must expand them to include additional context in each diff section
 *
 * note that the result of pairLines is a minimal set (e.g., top line is a change and bottom line is also change)
 * the logic of this function is relatively straightforward, but also accounts for cases where the lines that are passed in
 * are already exceeding a side's max (happens when content is inserted at the end of a file)
 *
 * hence, we need the number of lines in both the left side and the right side to be passed in
 */
export function expandLines(
  pairs: LinePair[],
  leftMax: number,
  rightMax: number,
): LinePair[] {
  let i = 0;
  while (i < pairs.length && (pairs[i][0] === null || pairs[i][1] === null))
    i++;

  if (i === pairs.length) {
    // lhs or rhs-only pairs
    const offset = pairs.length;
    // biome-ignore lint/style/noNonNullAssertion: rhs is non-null when lhs is absent in one-sided case
    const first = pairs[0][0] || pairs[0][1]!;

    // expand lines before
    const context: LinePair[] = [];
    const startLine = Math.max(0, first - CONTEXT_LINES - 1);
    for (let j = startLine; j < first; j++) {
      if (j >= leftMax || j >= rightMax) continue;
      context.push([j, j]);
    }
    pairs.unshift(...context);

    // expand lines after
    // biome-ignore lint/style/noNonNullAssertion: pairs is non-empty here (context was just added)
    const [lastLeft, lastRight] = pairs.at(-1)!;
    // biome-ignore lint/style/noNonNullAssertion: rhs is non-null when lastLeft is null in one-sided case
    const lastValue = lastLeft !== null ? lastLeft : lastRight!;

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
  } else {
    // at least one matched lines in the pairs
    const [leftNullsBefore, rightNullsBefore] = countNulls(pairs.slice(0, i));
    // biome-ignore lint/style/noNonNullAssertion: pairs[i] is a matched anchor with both sides non-null
    const topLeft = pairs[i][0]! - leftNullsBefore;
    // biome-ignore lint/style/noNonNullAssertion: pairs[i] is a matched anchor with both sides non-null
    const topRight = pairs[i][1]! - rightNullsBefore;

    const context: LinePair[] = [];
    for (let j = 1; j <= CONTEXT_LINES; j++) {
      const left = topLeft - j;
      const right = topRight - j;
      if (left < 0 || right < 0) {
        break;
      }
      context.push([left, right]);
    }
    context.reverse();
    pairs.unshift(...context);

    // find closest match from the ends
    let j = pairs.length - 1;
    while (j >= 0 && (pairs[j][0] === null || pairs[j][1] === null)) j--;

    // Count non-null entries after last matched line
    const [leftNullsAfter, rightNullsAfter] = countNulls(pairs.slice(j + 1));
    // biome-ignore lint/style/noNonNullAssertion: pairs[j] is the last matched anchor with both sides non-null
    const bottomLeft = pairs[j][0]! + leftNullsAfter;
    // biome-ignore lint/style/noNonNullAssertion: pairs[j] is the last matched anchor with both sides non-null
    const bottomRight = pairs[j][1]! + rightNullsAfter;

    for (let k = 1; k <= CONTEXT_LINES; k++) {
      const left = bottomLeft + k;
      const right = bottomRight + k;
      if (left >= leftMax || right >= rightMax) {
        break;
      }
      pairs.push([left, right]);
    }
  }

  return pairs;
}

function countNulls(pairs: LinePair[]): [number, number] {
  let leftCount = 0;
  let rightCount = 0;
  for (const pair of pairs) {
    if (pair[0] !== null) leftCount++;
    if (pair[1] !== null) rightCount++;
  }
  return [leftCount, rightCount];
}

// ============================================================================
// changed-line sets
// ============================================================================

/**
 * collects the set of changed line numbers per side
 *
 * used by renderSpans to tag added/removed lines for whole-line background coloring
 */
export function createChangeMaps(hunks: DiffHunk[]): {
  leftLines: Set<number>;
  rightLines: Set<number>;
} {
  const leftLines = new Set<number>();
  const rightLines = new Set<number>();
  for (const hunk of hunks) {
    for (const line of hunk) {
      if (line.lhs) leftLines.add(line.lhs.line_number);
      if (line.rhs) rightLines.add(line.rhs.line_number);
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
