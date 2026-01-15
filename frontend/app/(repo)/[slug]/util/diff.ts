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

/**
 * Align chunks so both sides have the same number of lines.
 * Only renders the chunk lines, no context.
 */
export function alignFiles(
  left: RepositoryFile,
  right: RepositoryFile,
  chunks: DiffChunk[],
): { leftContent: string; rightContent: string } {
  // so some logic like iterate chunk by chunk
  // for each chunk, we do the following
  // if the chunk has lhs or rhs only, don't do anything.
  // if the chunk has lhs and rhs, and both are the same lines, don't do anything
  // if the chunk has lhs and rhs and they have different lines, then pad the smaller side by the difference
  // for now, don't return full left content and full right content, but just do it for me
  //
  const leftFileLines = left.content.split("\n");
  const rightFileLines = right.content.split("\n");

  let leftTotalPadding = 0;
  let rightTotalPadding = 0;

  for (const chunk of chunks) {
    const [chunkStart, chunkEnd] = getChunkRange(chunk);

    const leftChunkLines = new Array(chunkEnd - chunkStart + 1);
    const rightChunkLines = new Array(chunkEnd - chunkStart + 1);

    let leftPadding = leftTotalPadding;
    let rightPadding = rightTotalPadding;

    for (const line of chunk) {
      if (line.lhs && line.rhs) {
        // not accumulative, but take the results rather
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

    let leftPointer = chunkStart + rightTotalPadding;
    for (let i = 0; i < leftChunkLines.length; i++) {
      if (!leftChunkLines[i]) {
        leftChunkLines[i] = leftFileLines[leftPointer++];
      }
    }

    let rightPointer = chunkStart + leftTotalPadding;
    for (let i = 0; i < rightChunkLines.length; i++) {
      if (!rightChunkLines[i]) {
        rightChunkLines[i] = rightFileLines[rightPointer++];
      }
    }

    leftTotalPadding += leftPadding - leftTotalPadding;
    rightTotalPadding += rightPadding - rightTotalPadding;

    console.log("LEFT:");
    for (let i = 0; i < leftChunkLines.length; i++) {
      console.log(leftChunkLines[i]);
    }

    console.log("RIGHT:");
    for (let i = 0; i < rightChunkLines.length; i++) {
      console.log(rightChunkLines[i]);
    }
  }

  return {
    leftContent: left?.content || "",
    rightContent: right?.content || "",
  };
}
