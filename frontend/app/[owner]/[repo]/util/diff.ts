import type { DiffChange, DiffHunk, DiffLine } from "@/lib/dto";

export type LinePair = [number | null, number | null];
export const CONTEXT_LINES = 4;

// ============================================================================
// diffing utils
// ============================================================================

/**
 * there's a bug in difftastic where lhs removals can be placed after rhs additions
 * which leads to lines showing in weird sequential order
 *
 * so to avoid that, and enable consistent processing in mergeHunks and downstream, sort hunks here
 */
export function sortHunks(hunks: DiffHunk[]): DiffHunk[] {
  const getStartingLine = (hunk: DiffHunk): number => {
    const firstLine = hunk[0];
    return firstLine.lhs
      ? firstLine.lhs.line_number
      : firstLine.rhs!.line_number;
  };

  const sortedHunks = [...hunks].sort((a, b) => {
    return getStartingLine(a) - getStartingLine(b);
  });

  return sortedHunks;
}

/**
 * the set of hunks that difftastic returns is the minimal set
 * e.g., if line 4 was modified, then only line 4 will be returned in that chunk.
 *
 * difftastic does some chunk grouping as well (based off a line number of 4), but we need to do additional merging
 * as that does not account for the context lines we inject around each chunk
 * e.g., difftastic will return lines 4 and lines 9 as two hunk, if we expand each to include 4 lines around it, they now overlap and should be merged
 *
 * note that this occurs prior to any pairing of lines, as if we were to try and merge lines after they were paired,
 * we _may_ find logically conflicting pairs (e.g., we accounted for gaps in one hunk, but are unaware in the second)
 */
export function mergeHunks(hunks: DiffHunk[]): DiffHunk[] {
  if (hunks.length <= 1) return hunks;

  const sortedHunks = sortHunks(hunks);
  const result: DiffHunk[] = [];
  let current = [...sortedHunks[0]];

  for (let i = 1; i < sortedHunks.length; i++) {
    const next = sortedHunks[i];

    if (hunksOverlap(current, next)) {
      current = [...current, ...next];
    } else {
      result.push(current);
      current = [...next];
    }
  }

  result.push(current);
  return result;
}

function hunksOverlap(left: DiffHunk, right: DiffHunk): boolean {
  const lastLine = left[left.length - 1];
  const firstLine = right[0];

  const [lastLhs, lastRhs] = getEffectivePosition(
    lastLine,
    left,
    left.length - 1,
  );
  const [firstLhs, firstRhs] = getEffectivePosition(firstLine, right, 0);

  // Overlap if either side's difference is within CONTEXT_LINES * 2
  const lhsDiff = firstLhs - lastLhs;
  const rhsDiff = firstRhs - lastRhs;

  return lhsDiff <= CONTEXT_LINES * 2 || rhsDiff <= CONTEXT_LINES * 2;
}

// todo: rename this
function getEffectivePosition(
  line: DiffLine,
  hunk: DiffHunk,
  lineIndex: number,
): [number, number] {
  if (line.lhs && line.rhs) {
    return [line.lhs.line_number, line.rhs.line_number];
  }

  // look for an anchor (a line with both lhs and rhs above) and if found, use that
  for (let i = lineIndex - 1; i >= 0; i--) {
    const anchor = hunk[i];
    if (anchor.lhs && anchor.rhs) {
      return [anchor.lhs.line_number, line.rhs!.line_number];
    }
  }

  // else there is no anchor, must be a lhs only or a rhs only line
  const firstLine = hunk[0];
  if (line.lhs) {
    const firstLhs = firstLine.lhs?.line_number ?? line.lhs.line_number;
    return [line.lhs.line_number, firstLhs - 1];
  } else {
    const firstRhs = firstLine.rhs?.line_number ?? line.rhs!.line_number;
    return [firstRhs - 1, line.rhs!.line_number];
  }
}

/**
 * a tad complicated and heuristic function.
 *
 * difft --display JSON returns hunks, which are sequence of changed, added, removed lines separate a 4 line radius
 * each hunk composes of a list of lines, and a line can be of three forms:
 * - added [rhs only]
 * - removed [lhs only]
 * - modified [lhs + rhs]
 *
 * difft _also_ returns line numbers along with each diff line, which is important in particular for
 * modified lines as it indicates the matching of each line sequence, e.g., [15, 20] means that this line should be aligned to file line 15 on the left and 20 on the rhs.
 *
 * this schema allows for difft to match seeming unrelated and non-contiguous lines, which allows for smarter syntax based mapping, but makes the schema's interpretation ambiguous.
 * for example, with a pair of [15, 20], to have the result be aligned in the UI, we need to pad the left side by 5 lines.
 *
 * but the question of where to pad is ambiguous (e.g., could be at the top of the file, could be right before, could be somewhere in the middle).
 * so this code attempts to re-construct what difft is doing in the terminal to return sensible alignment and formatting.
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
 * - each side must contain the full range of indices from min, max provided in difft (inclusive)
 * - each side must be monotonically increasing, exlcuding null sentinels
 * this also implies that the size of the list _may_ be greater than the full range of indices as a result of padding, which is fine.
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
    const value = isLhsOnly ? pair[0]! : pair[1]!;
    if (result.length > 0) {
      const lastValue = isLhsOnly ? result.at(-1)![0]! : result.at(-1)![1]!;
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
  const lhsGap = endAnchor[0]! - startAnchor[0]! - 1;
  const rhsGap = endAnchor[1]! - startAnchor[1]! - 1;

  const existingLhsNulls = entriesBetween.filter((p) => p[0] === null).length;
  const existingRhsNulls = entriesBetween.filter((p) => p[1] === null).length;

  // diff > 0 means we need more lhs nulls, diff < 0 means we need more rhs nulls
  const diff = rhsGap - lhsGap - existingLhsNulls + existingRhsNulls;

  const result: LinePair[] = [];
  let leftPos = startAnchor[0]! + 1;
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

// ============================================================================
// highlighting utils
// ============================================================================

/**
 * parses diff hunks into line number indexed change maps
 *
 * used by syntax highlighting functions
 */
export function createChangeMaps(hunks: DiffHunk[]): {
  leftChangeMap: Map<number, DiffChange[]>;
  rightChangeMap: Map<number, DiffChange[]>;
} {
  const leftLines = new Map<number, DiffChange[]>();
  const rightLines = new Map<number, DiffChange[]>();
  for (const hunk of hunks) {
    for (const line of hunk) {
      if (line.lhs) {
        leftLines.set(line.lhs.line_number, line.lhs.changes);
      }
      if (line.rhs) {
        rightLines.set(line.rhs.line_number, line.rhs.changes);
      }
    }
  }

  return { leftChangeMap: leftLines, rightChangeMap: rightLines };
}
