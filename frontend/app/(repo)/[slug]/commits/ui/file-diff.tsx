import type { Element, Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import { inferLanguage } from "@/(repo)/[slug]/util";
import type { DiffChunk, RepositoryFile, RepositoryFileDiff } from "@/lib/dto";
import {
  buildLineTypeMap,
  countLines,
  type DiffLineType,
  expandWithContext,
  extractLineNumbers,
  isLineInRanges,
  type LineRange,
} from "../lib/diff-utils";
import { ChunkSeparator } from "./chunk-separator";
import { DiffLine } from "./diff-line";

const CONTEXT_LINES = 5;

async function renderDiffSide(
  file: RepositoryFile | undefined,
  chunks: DiffChunk[],
  side: "lhs" | "rhs",
): Promise<JSX.Element | null> {
  if (!file?.content) {
    return null;
  }

  const totalLines = countLines(file.content);

  // Extract line numbers and expand with context
  const changedLines = extractLineNumbers(chunks, side);
  const visibleRanges = expandWithContext(
    changedLines,
    CONTEXT_LINES,
    totalLines,
  );

  // Build line type map for styling
  const lineTypeMap = buildLineTypeMap(chunks, side);

  const hast = await codeToHast(file.content, {
    lang: inferLanguage(file.path) ?? "plaintext",
    theme: "vitesse-light",
    transformers: [
      {
        pre(node) {
          this.addClassToHast(node, "outline-none");
        },
        code(node) {
          this.addClassToHast(node, "flex flex-col");
        },
        line(node, lineNumber) {
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;

          // Mark visibility
          const isVisible = isLineInRanges(lineNumber, visibleRanges);
          node.properties["data-visible"] = isVisible;

          // Set diff type if this line is in the chunks
          const diffType = lineTypeMap.get(lineNumber);
          if (diffType) {
            node.properties["data-diff-type"] = diffType;
          }
        },
      },
    ],
  });
  // Insert separators between non-contiguous ranges
  const processedHast = insertChunkSeparators(hast, visibleRanges);

  return toJsxRuntime(processedHast, {
    Fragment,
    jsx,
    jsxs,
    components: {
      diffline: (props) => <DiffLine {...props} />,
      chunkseparator: (props) => <ChunkSeparator {...props} />,
    },
  }) as JSX.Element;
}

/**
 * Insert separator elements between non-contiguous line ranges
 */
function insertChunkSeparators(hast: Root, ranges: LineRange[]): Root {
  if (ranges.length <= 1) return hast;

  // Find the <pre> -> <code> element that contains lines
  const preElement = hast.children.find(
    (c): c is Element => c.type === "element" && c.tagName === "pre",
  );
  if (!preElement) return hast;

  const codeElement = preElement.children.find(
    (c): c is Element => c.type === "element" && c.tagName === "code",
  );
  if (!codeElement) return hast;

  const newChildren: typeof codeElement.children = [];
  let currentRangeIndex = 0;
  let lastLineNumber = 0;

  for (const child of codeElement.children) {
    if (child.type !== "element") {
      newChildren.push(child);
      continue;
    }

    const lineNumber = child.properties?.["data-line-number"] as number;
    if (!lineNumber) {
      newChildren.push(child);
      continue;
    }

    // Check if we've crossed into a new range (there's a gap)
    while (
      currentRangeIndex < ranges.length - 1 &&
      lineNumber > ranges[currentRangeIndex].end
    ) {
      // We're past the current range, insert separator
      const gapStart = ranges[currentRangeIndex].end;
      const gapEnd = ranges[currentRangeIndex + 1].start;
      const hiddenCount = gapEnd - gapStart - 1;

      if (hiddenCount > 0) {
        newChildren.push({
          type: "element",
          tagName: "chunkseparator",
          properties: {
            "data-hidden-count": hiddenCount,
          },
          children: [],
        });
      }

      currentRangeIndex++;
    }

    lastLineNumber = lineNumber;
    newChildren.push(child);
  }

  codeElement.children = newChildren;
  return hast;
}

export async function FileDiff({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, chunks } = diff;
  const path = left?.path || right?.path;

  // If no chunks, show a message
  if (!chunks || chunks.length === 0) {
    return (
      <div className="flex flex-col w-full">
        <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
          {path}
        </div>
        <div className="flex items-center justify-center p-4 text-muted-foreground text-sm">
          No changes to display
        </div>
      </div>
    );
  }

  const [leftContent, rightContent] = await Promise.all([
    renderDiffSide(left, chunks, "lhs"),
    renderDiffSide(right, chunks, "rhs"),
  ]);

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
        {path}
      </div>
      <div className="flex flex-row w-full">
        <div className="flex flex-col w-1/2 text-sm font-mono overflow-x-auto">
          {leftContent ?? (
            <div className="flex items-center justify-center p-4 text-muted-foreground text-sm italic">
              File added
            </div>
          )}
        </div>
        <div className="flex flex-col w-1/2 text-sm font-mono overflow-x-auto border-border border-l">
          {rightContent ?? (
            <div className="flex items-center justify-center p-4 text-muted-foreground text-sm italic">
              File deleted
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
