import type { DiffChunk, RepositoryFile } from "@/lib/dto";

export type LinePair = [number | null, number | null]; // null represents SENTINEL

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
export function pairLines(chunk: DiffChunk): LinePair[] {
  const chunkPairs: LinePair[] = [];

  // we first add all paired lines (those that are matched) and use those as anchors to generate the full alignment
  for (const line of chunk) {
    if (line.lhs && line.rhs) {
      chunkPairs.push([line.lhs.line_number, line.rhs.line_number]);
    }
  }

  // then insert lhs only lines by sorted order of the left index
  for (const line of chunk) {
    if (line.lhs && !line.rhs) {
      insertLhsInOrder(chunkPairs, line.lhs.line_number);
    }
  }

  // then do the same for rhs
  for (const line of chunk) {
    if (!line.lhs && line.rhs) {
      insertRhsInOrder(chunkPairs, line.rhs.line_number);
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
  const lastPairIdx = chunkPairs.findLastIndex(
    (p) => p[0] !== null && p[1] !== null,
  );
  if (lastPairIdx !== -1) {
    const anchor = chunkPairs[lastPairIdx];
    linePairs.unshift(anchor);

    // biome-ignore lint/style/noNonNullAssertion: anchor verified non-null by findLastIndex
    let leftPos = anchor[0]!;
    // biome-ignore lint/style/noNonNullAssertion: anchor verified non-null by findLastIndex
    let rightPos = anchor[1]!;

    for (let i = lastPairIdx - 1; i >= 0; i--) {
      const entry = chunkPairs[i];
      while (true) {
        const leftMatches = entry[0] !== null && leftPos - 1 === entry[0];
        const rightMatches = entry[1] !== null && rightPos - 1 === entry[1];

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
    // No paired lines (all one-sided) - just use chunkPairs directly
    // and let the gap-filling logic below handle filling in missing lines
    linePairs.push(...chunkPairs);
  }

  // now, we fill in all the gaps remaining in the list of pairs with sentinel entries by iterating backwards
  const [minLine, maxLine] = getChunkRange(chunk);

  let hasGaps = true;
  while (hasGaps) {
    hasGaps = false;

    // check start: prepend if either side starts before minLine
    const first = linePairs[0];
    if (first[0] !== null && first[0] > minLine) {
      linePairs.unshift([first[0] - 1, null]);
      hasGaps = true;
      continue;
    }
    if (first[1] !== null && first[1] > minLine) {
      linePairs.unshift([null, first[1] - 1]);
      hasGaps = true;
      continue;
    }

    // check the middle by iterating backwards
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
    if (hasGaps) continue;

    // check end: append if either side ends before maxLine
    const last = linePairs[linePairs.length - 1];
    if (
      (last[0] !== null && last[0] < maxLine) ||
      (last[1] !== null && last[1] < maxLine)
    ) {
      linePairs.push([
        last[0] !== null ? last[0] + 1 : null,
        last[1] !== null ? last[1] + 1 : null,
      ]);
      hasGaps = true;
    }
  }

  return linePairs;
}

/**
 * - visible lines are 0-indexed set of all visible lines in chunks
 * - sentinel counts are the number of sentinels that should be inserted _before_ a line
 */
export function processChunks(
  _left: RepositoryFile,
  _right: RepositoryFile,
  chunks: DiffChunk[],
): {
  leftVisibleLines: Set<number>;
  leftSentinelCounts: Map<number, number>;
  rightVisibleLines: Set<number>;
  rightSentinelCounts: Map<number, number>;
} {
  const leftVisibleLines = new Set<number>();
  const rightVisibleLines = new Set<number>();
  const leftSentinelCounts = new Map<number, number>();
  const rightSentinelCounts = new Map<number, number>();

  for (const chunk of chunks) {
    const linePairs = pairLines(chunk);

    let leftSentinelCount = 0;
    let rightSentinelCount = 0;

    for (const [left, right] of linePairs) {
      if (left === null) {
        leftSentinelCount++;
      } else {
        leftVisibleLines.add(left);
        if (leftSentinelCount > 0) {
          leftSentinelCounts.set(
            left,
            (leftSentinelCounts.get(left) ?? 0) + leftSentinelCount,
          );
          leftSentinelCount = 0;
        }
      }

      if (right === null) {
        rightSentinelCount++;
      } else {
        rightVisibleLines.add(right);
        if (rightSentinelCount > 0) {
          rightSentinelCounts.set(
            right,
            (rightSentinelCounts.get(right) ?? 0) + rightSentinelCount,
          );
          rightSentinelCount = 0;
        }
      }
    }
  }

  return {
    leftVisibleLines,
    leftSentinelCounts,
    rightVisibleLines,
    rightSentinelCounts,
  };
}

function getChunkRange(chunk: DiffChunk): [number, number] {
  let min = Infinity;
  let max = -Infinity;

  for (const line of chunk) {
    if (line.lhs) {
      min = Math.min(min, line.lhs.line_number);
      max = Math.max(max, line.lhs.line_number);
    }
    if (line.rhs) {
      min = Math.min(min, line.rhs.line_number);
      max = Math.max(max, line.rhs.line_number);
    }
  }

  return min === Infinity ? [0, 0] : [min, max];
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
