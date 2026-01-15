import type { DiffChunk, DiffLine } from "@/lib/dto";

export type LineRange = {
  start: number;
  end: number;
};

export type DiffLineType = "added" | "removed" | "modified" | "context";

/**
 * Extract all line numbers from chunks for a given side
 */
export function extractLineNumbers(
  chunks: DiffChunk[],
  side: "lhs" | "rhs",
): number[] {
  const lineNumbers: number[] = [];

  for (const chunk of chunks) {
    for (const line of chunk) {
      const sideData = line[side];
      if (sideData) {
        lineNumbers.push(sideData.line_number);
      }
    }
  }

  return lineNumbers;
}

/**
 * Expand line numbers with context padding and return ranges
 */
export function expandWithContext(
  lineNumbers: number[],
  contextLines: number,
  maxLine: number,
): LineRange[] {
  if (lineNumbers.length === 0) return [];

  // Expand each line with context
  const expanded = lineNumbers.flatMap((line) => {
    const start = Math.max(1, line - contextLines);
    const end = Math.min(maxLine, line + contextLines);
    return { start, end };
  });

  return mergeRanges(expanded);
}

/**
 * Merge overlapping or adjacent ranges
 */
export function mergeRanges(ranges: LineRange[]): LineRange[] {
  if (ranges.length === 0) return [];

  // Sort by start
  const sorted = [...ranges].sort((a, b) => a.start - b.start);
  const merged: LineRange[] = [sorted[0]];

  for (let i = 1; i < sorted.length; i++) {
    const current = sorted[i];
    const last = merged[merged.length - 1];

    // Merge if overlapping or adjacent
    if (current.start <= last.end + 1) {
      last.end = Math.max(last.end, current.end);
    } else {
      merged.push(current);
    }
  }

  return merged;
}

/**
 * Build a map from line number to diff type for styling purposes
 */
export function buildLineTypeMap(
  chunks: DiffChunk[],
  side: "lhs" | "rhs",
): Map<number, DiffLineType> {
  const map = new Map<number, DiffLineType>();

  for (const chunk of chunks) {
    for (const line of chunk) {
      const sideData = line[side];
      if (!sideData) continue;

      const lineNumber = sideData.line_number;
      const otherSide = side === "lhs" ? line.rhs : line.lhs;

      let type: DiffLineType;
      if (!otherSide) {
        // Line only exists on one side
        type = side === "lhs" ? "removed" : "added";
      } else {
        // Line exists on both sides - it's modified
        type = "modified";
      }

      map.set(lineNumber, type);
    }
  }

  return map;
}

/**
 * Check if a line number is within any of the given ranges
 */
export function isLineInRanges(
  lineNumber: number,
  ranges: LineRange[],
): boolean {
  return ranges.some((r) => lineNumber >= r.start && lineNumber <= r.end);
}

/**
 * Count total lines in file content
 */
export function countLines(content: string): number {
  return content.split("\n").length;
}

export const SENTINEL_LINE = "---";

/**
 * Align chunks so both sides have the same number of lines.
 * Only renders the chunk lines, no context.
 */
export function alignFiles(
  lhsContent: string,
  rhsContent: string,
  chunks: DiffChunk[],
): { leftContent: string; rightContent: string } {
  const lhsLines = lhsContent.split("\n");
  const rhsLines = rhsContent.split("\n");

  const outputLhs: string[] = [];
  const outputRhs: string[] = [];

  for (const chunk of chunks) {
    for (const line of chunk) {
      if (line.lhs && line.rhs) {
        outputLhs.push(lhsLines[line.lhs.line_number]);
        outputRhs.push(rhsLines[line.rhs.line_number]);
      } else if (line.lhs) {
        outputLhs.push(lhsLines[line.lhs.line_number]);
        outputRhs.push(SENTINEL_LINE);
      } else if (line.rhs) {
        outputLhs.push(SENTINEL_LINE);
        outputRhs.push(rhsLines[line.rhs.line_number]);
      }
    }
  }

  return {
    leftContent: outputLhs.join("\n"),
    rightContent: outputRhs.join("\n"),
  };
}
