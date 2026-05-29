import { structuredPatch } from "diff";
import type { DiffHunk, DiffPair } from "./types";

const CONTEXT_LINES = 4;
const MERGE_THRESHOLD = 8;

/**
 * jsdiff's structuredPath generally returns comparable output to unified patch algorithms
 *
 * note that context lines here will show up and down even for an addition only or removal change
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
    let oldLine = hunk.oldStart - 1;
    let newLine = hunk.newStart - 1;
    const pairs: DiffPair[] = [];
    const removedLines = new Set<number>();
    const addedLines = new Set<number>();

    for (const raw of hunk.lines) {
      const marker = raw[0];
      if (marker === "-") {
        pairs.push([oldLine, null]);
        removedLines.add(oldLine);
        oldLine++;
      } else if (marker === "+") {
        pairs.push([null, newLine]);
        addedLines.add(newLine);
        newLine++;
      } else if (marker === " ") {
        pairs.push([oldLine, newLine]);
        oldLine++;
        newLine++;
      }
    }
    result.push({ pairs: alignPairs(pairs), removedLines, addedLines });
  }

  return mergeHunks(result);
}

/**
 * this function does two things:
 * - zip lines (e.g., - a - b + c + d) => pairs a with c and b with d
 * - pad sentinels if line count mismatches (e.g., 3 removals, 4 additions)
 */
function alignPairs(pairs: DiffPair[]): DiffPair[] {
  const result: DiffPair[] = [];
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

  for (const [L, R] of pairs) {
    if (L !== null && R !== null) {
      flush();
      result.push([L, R]);
    } else if (L !== null) {
      pendingLhs.push(L);
    } else if (R !== null) {
      pendingRhs.push(R);
    }
  }
  flush();

  return result;
}

/**
 * regardless of the context window we set, there are unfortunate cases
 *
 * where we will have a small hidden section in between two hunks (e.g,. hiding two lines)
 * this is ugly, so we merge them
 *
 * since hunks are alraedy paired before this, we can just increment the counts in between each hunk
 */
function mergeHunks(hunks: DiffHunk[]): DiffHunk[] {
  if (hunks.length <= 1) return hunks;
  const result: DiffHunk[] = [hunks[0]];
  for (let i = 1; i < hunks.length; i++) {
    const prev = result[result.length - 1];
    const next = hunks[i];
    const prevLast = prev.pairs.at(-1);
    const nextFirst = next.pairs[0];
    const prevL = prevLast?.[0];
    const prevR = prevLast?.[1];
    const nextL = nextFirst?.[0];

    if (prevL == null || prevR == null || nextL == null) {
      result.push(next);
      continue;
    }

    const gap = nextL - prevL - 1;
    if (gap <= 0 || gap > MERGE_THRESHOLD) {
      result.push(next);
      continue;
    }

    const fill: DiffPair[] = [];
    for (let k = 1; k <= gap; k++) fill.push([prevL + k, prevR + k]);

    result[result.length - 1] = {
      pairs: [...prev.pairs, ...fill, ...next.pairs],
      removedLines: new Set([...prev.removedLines, ...next.removedLines]),
      addedLines: new Set([...prev.addedLines, ...next.addedLines]),
    };
  }
  return result;
}
