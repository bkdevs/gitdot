import type { DiffHunk } from "@/lib/dto";

// ============================================================================
// pairLines
// ============================================================================

export type LinePair = [number | null, number | null];

/**
 * a tad complicated and heuristic function.
 *
 * diffd --display JSON returns hunks, which are sequence of changed, added, removed lines separate a 4 line radius
 * each hunk composes of a list of lines, and a line can be of three forms:
 * - added [rhs only]
 * - removed [lhs only]
 * - modified [lhs + rhs]
 *
 * diffd _also_ returns line numbers along with each diff line, which is important in particular for
 * modified lines as it indicates the matching of each line sequence, e.g., [15, 20] means that this line should be aligned to file line 15 on the left and 20 on the rhs.
 *
 * this schema allows for diffd to match seeming unrelated and non-contiguous lines, which allows for smarter syntax based mapping, but makes the schema's interpretation ambiguous.
 * for example, with a pair of [15, 20], to have the result be aligned in the UI, we need to pad the left side by 5 lines.
 *
 * but the question of where to pad is ambiguous (e.g., could be at the top of the file, could be right before, could be somewhere in the middle).
 * so this code attempts to re-construct what diffd is doing in the terminal to return sensible alignment and formatting.
 *
 * output:
 *  - LinePair[], a list of line numbers that indicate which left line maps to what right line (and what lines should be sentinelled)
 *
 * example output:
 *  - [1, 2] # showing offset at beginning of file
 *  - [2, 3]
 *  - [null, 4] # indicating added on rhs
 *  - [3, 5]
 *  - [4, null] # indicating removal on lhs
 *
 * a few invariants must hold with this list, which make reasoning about it easier:
 * - each side must contain the full range of indices from min, max provided in diffd (inclusive)
 * - each side must be monotonically increasing, exlcuding null sentinels
 * this also implies that the size of the list _may_ be greater than the full range of indices as a result of padding, which is fine.
 */
export function pairLines(hunk: DiffHunk): LinePair[] {
  const hunkPairs: LinePair[] = [];

  // we first add all paired lines (those that are matched) and use those as anchors to generate the full alignment
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

  const linePairs: LinePair[] = [];

  // TODO: the algorithm below is naive and can be made better
  // rather than thinking of "filling in gaps", it is better to identify ranges between paired lines where sentinels must be inserted
  // and then attempting to insert the sentinels in places where it makes sense (e.g., grou pgaps together) and/or try and do some word-matching thing as well
  //
  // to illustrate, say you have the pairs
  // [1, 1]
  // [2, 3]
  //
  // because [1, 1] and [2, 3] are both matched lines,
  // they constitute a range in which a sentinel _may_ be added and since 2 - 1 != 3 - 1, a sentinel must be haddded
  //
  // this also applies to ranges that have lhs / rhs only provided already, e.g.,
  //
  // [1, 1]
  // [null, 3]
  // [5, 7]
  //
  // because (7 - 1) - (5 - 1) - 1 (from the null) == 1, we still need to add one more sentinel on the left side.
  // we have several solutions here, we could do
  // [1, 1]
  // [2, 2]
  // [null, 3]
  // [3, 4]
  // [4, 5]
  // [null, 6]
  // [5, 7]
  //
  //
  // but that is potentially ugly as the sentinels are on split lines.
  // so we can write a potentially better looking algorithm by identifying the ranges
  // where sentinels must be placed and then trying to place them smartly.
  //
  // as of now, the below implementation just fills in lines from back to top and places sentinels a bit arbitrarily.
  const lastPairIdx = hunkPairs.findLastIndex(
    (p) => p[0] !== null && p[1] !== null,
  );
  if (lastPairIdx !== -1) {
    const anchor = hunkPairs[lastPairIdx];
    linePairs.unshift(anchor);

    // biome-ignore lint/style/noNonNullAssertion: anchor verified non-null by findLastIndex
    let leftPos = anchor[0]!;
    // biome-ignore lint/style/noNonNullAssertion: anchor verified non-null by findLastIndex
    let rightPos = anchor[1]!;

    for (let i = lastPairIdx - 1; i >= 0; i--) {
      const entry = hunkPairs[i];
      while (true) {
        const leftMatches = entry[0] !== null && leftPos - 1 === entry[0];
        const rightMatches = entry[1] !== null && rightPos - 1 === entry[1];

        console.log(" we are in an infinite loop"); // TODO: fix

        if (leftMatches || rightMatches) {
          if (entry[0] !== null) leftPos = entry[0];
          if (entry[1] !== null) rightPos = entry[1];
          linePairs.unshift(entry);
          break;
        }

        leftPos--;
        rightPos--;
        linePairs.unshift([leftPos, rightPos]);
      }
    }
  } else {
    // No paired lines (all one-sided) - just use hunkPairs directly
    // and let the gap-filling logic below handle filling in missing lines
    linePairs.push(...hunkPairs);
  }

  let hasGaps = true;
  while (hasGaps) {
    hasGaps = false;
    for (let i = linePairs.length - 1; i > 0; i--) {
      const current = linePairs[i];
      const prev = linePairs[i - 1];

      if (current[1] !== null && prev[1] !== null && current[1] - prev[1] > 1) {
        linePairs.splice(i, 0, [null, current[1] - 1]);
        hasGaps = true;
        break;
      }
      if (current[0] !== null && prev[0] !== null && current[0] - prev[0] > 1) {
        linePairs.splice(i, 0, [current[0] - 1, null]);
        hasGaps = true;
        break;
      }
    }
  }
  return linePairs;
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

// ============================================================================
// expandLines
// ============================================================================

const CONTEXT_LINES = 4;

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
    const [lastLeft, lastRight] = pairs.at(-1)!;
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
    const topLeft = pairs[i][0]! - leftNullsBefore;
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
    const bottomLeft = pairs[j][0]! + leftNullsAfter;
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
