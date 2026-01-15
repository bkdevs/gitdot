import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import {
  alignFiles,
  buildLineTypeMap,
  countLines,
  expandWithContext,
  extractLineNumbers,
  inferLanguage,
  isLineInRanges,
} from "@/(repo)/[slug]/util";
import type { DiffChunk, RepositoryFile, RepositoryFileDiff } from "@/lib/dto";
import { DiffLine } from "./diff-line";

const CONTEXT_LINES = 5;

async function renderDiffSide(
  language: string,
  content: string,
  chunks: DiffChunk[],
  side: "lhs" | "rhs",
): Promise<JSX.Element | null> {
  const totalLines = countLines(content);

  // Extract line numbers and expand with context
  const changedLines = extractLineNumbers(chunks, side);
  const visibleRanges = expandWithContext(
    changedLines,
    CONTEXT_LINES,
    totalLines,
  );

  // Build line type map for styling
  const lineTypeMap = buildLineTypeMap(chunks, side);

  const hast = await codeToHast(content, {
    lang: language,
    theme: "vitesse-light",
    transformers: [
      {
        pre(node) {
          this.addClassToHast(node, "outline-none");
        },
        code(node) {
          // required as shiki by default renders code as a line
          this.addClassToHast(node, "flex flex-col");
        },
        line(node, lineNumber) {
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;

          // Mark visibility
          const isVisible = isLineInRanges(lineNumber, visibleRanges);
          node.properties["data-visible"] = true;

          // Set diff type if this line is in the chunks
          const diffType = lineTypeMap.get(lineNumber);
          if (diffType) {
            node.properties["data-diff-type"] = diffType;
          }
        },
      },
    ],
  });

  return toJsxRuntime(hast, {
    Fragment,
    jsx,
    jsxs,
    components: {
      diffline: (props) => <DiffLine {...props} />,
    },
  }) as JSX.Element;
}

export async function FileDiff({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, chunks } = diff;
  const path = left?.path || right?.path;
  if (!path || !chunks || chunks.length === 0) {
    throw new Error("File path or chunks are missing");
  }

  const language = inferLanguage(path) || "plaintext";
  const { leftContent, rightContent } = alignFiles(
    left?.content || "",
    right?.content || "",
    chunks,
  );

  console.log(JSON.stringify(chunks, null, 2));

  const [leftComponent, rightComponent] = await Promise.all([
    renderDiffSide(language, leftContent, chunks, "lhs"),
    renderDiffSide(language, rightContent, chunks, "rhs"),
  ]);

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
        {path}
      </div>
      <div className="flex flex-row w-full">
        <div className="flex flex-col w-1/2 text-sm font-mono overflow-x-auto">
          {leftComponent ?? (
            <div className="flex items-center justify-center p-4 text-muted-foreground text-sm italic">
              File added
            </div>
          )}
        </div>
        <div className="flex flex-col w-1/2 text-sm font-mono overflow-x-auto border-border border-l">
          {rightComponent ?? (
            <div className="flex items-center justify-center p-4 text-muted-foreground text-sm italic">
              File deleted
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
