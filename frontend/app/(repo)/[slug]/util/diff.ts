import type { DiffChunk, RepositoryFile } from "@/lib/dto";

export const SENTINEL_LINE = "---";

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

function processRemovedChunk(
  chunk: DiffChunk,
  leftLines: string[],
  leftOffset: number,
): [string[], string[]] {
  const [chunkStart, chunkEnd] = getChunkRange(chunk);

  const leftChunkLines = new Array(chunkEnd - chunkStart + 1);
  const rightChunkLines = new Array(chunkEnd - chunkStart + 1);

  for (let i = 0; i < leftChunkLines.length; i++) {
    leftChunkLines[i] = leftLines[chunkStart + i + leftOffset];
    rightChunkLines[i] = SENTINEL_LINE;
  }

  return [leftChunkLines, rightChunkLines];
}

function processAddedChunk(
  chunk: DiffChunk,
  rightLines: string[],
  rightOffset: number,
): [string[], string[]] {
  const [chunkStart, chunkEnd] = getChunkRange(chunk);

  const leftChunkLines = new Array(chunkEnd - chunkStart + 1);
  const rightChunkLines = new Array(chunkEnd - chunkStart + 1);

  for (let i = 0; i < rightChunkLines.length; i++) {
    leftChunkLines[i] = SENTINEL_LINE;
    rightChunkLines[i] = rightLines[chunkStart + i + rightOffset];
  }

  return [leftChunkLines, rightChunkLines];
}

function processModifiedChunk(
  chunk: DiffChunk,
  leftLines: string[],
  leftOffset: number,
  rightLines: string[],
  rightOffset: number,
): [string[], string[]] {
  const [chunkStart, chunkEnd] = getChunkRange(chunk);

  const leftChunkLines = new Array(chunkEnd - chunkStart + 1);
  const rightChunkLines = new Array(chunkEnd - chunkStart + 1);

  let leftPadding = leftOffset;
  let rightPadding = rightOffset;

  for (const line of chunk) {
    if (line.lhs && line.rhs) {
      let leftLine = line.lhs.line_number + leftPadding;
      let rightLine = line.rhs.line_number + rightPadding;

      while (leftLine < rightLine) {
        leftChunkLines[leftLine - chunkStart] = SENTINEL_LINE;
        leftPadding += 1;
        leftLine += 1;
      }
      while (rightLine < leftLine) {
        rightChunkLines[rightLine - chunkStart] = SENTINEL_LINE;
        rightPadding += 1;
        rightLine += 1;
      }
    } else if (line.lhs) {
      const rightLine = line.lhs.line_number + rightPadding;
      rightChunkLines[rightLine - chunkStart] = SENTINEL_LINE;
    } else if (line.rhs) {
      const leftLine = line.rhs.line_number + leftPadding;
      leftChunkLines[leftLine - chunkStart] = SENTINEL_LINE;
    }
  }

  let leftPointer = chunkStart + rightOffset;
  for (let i = 0; i < leftChunkLines.length; i++) {
    if (!leftChunkLines[i]) {
      leftChunkLines[i] = leftLines[leftPointer++];
    }
  }

  let rightPointer = chunkStart + leftOffset;
  for (let i = 0; i < rightChunkLines.length; i++) {
    if (!rightChunkLines[i]) {
      rightChunkLines[i] = rightLines[rightPointer++];
    }
  }

  return [leftChunkLines, rightChunkLines];
}

function processChunk(
  chunk: DiffChunk,
  leftLines: string[],
  rightLines: string[],
  leftOffset: number,
  rightOffset: number,
): [string[], string[]] {
  if (chunk.every((line) => line.lhs && !line.rhs)) {
    return processRemovedChunk(chunk, leftLines, leftOffset);
  } else if (chunk.every((line) => line.rhs && !line.lhs)) {
    return processAddedChunk(chunk, rightLines, rightOffset);
  } else {
    return processModifiedChunk(
      chunk,
      leftLines,
      leftOffset,
      rightLines,
      rightOffset,
    );
  }
}

export function alignFiles(
  left: RepositoryFile,
  right: RepositoryFile,
  chunks: DiffChunk[],
): { leftContent: string; rightContent: string } {
  const leftLines = left.content.split("\n");
  const rightLines = right.content.split("\n");

  let leftOffset = 0;
  let rightOffset = 0;

  for (const chunk of chunks) {
    const [leftChunkLines, rightChunkLines] = processChunk(
      chunk,
      leftLines,
      rightLines,
      leftOffset,
      rightOffset,
    );

    leftOffset += leftChunkLines.filter(
      (line) => line === SENTINEL_LINE,
    ).length;
    rightOffset += rightChunkLines.filter(
      (line) => line === SENTINEL_LINE,
    ).length;

    const columnWidth = 80; // Adjust as needed
    const separator = " | ";

    console.log("LEFT".padEnd(columnWidth) + separator + "RIGHT");
    console.log("-".repeat(columnWidth) + separator + "-".repeat(columnWidth));

    const maxLines = Math.max(leftChunkLines.length, rightChunkLines.length);

    for (let i = 0; i < maxLines; i++) {
      const left = (leftChunkLines[i] || "")
        .substring(0, columnWidth)
        .padEnd(columnWidth);
      const right = (rightChunkLines[i] || "").substring(0, columnWidth);
      console.log(left + separator + right);
    }
  }

  return {
    leftContent: left?.content || "",
    rightContent: right?.content || "",
  };
}
