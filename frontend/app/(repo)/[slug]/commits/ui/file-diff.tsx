import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import { getVisibleLines, inferLanguage } from "@/(repo)/[slug]/util";
import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffLine } from "./diff-line";

const CONTEXT_LINES = 5;

async function renderDiffSide(
  language: string,
  content: string,
  visibleLines: Set<number>
): Promise<JSX.Element | null> {
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
          node.properties["data-visible"] = visibleLines.has(lineNumber - 1);
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
  if (!path) {
    throw new Error("File path or chunks are missing");
  }
  if (!chunks || chunks.length === 0) {
    return (
      <div className="w-full">
        TK, this gets returned when files are deleted, created, or renamed
      </div>
    );
  }

  const language = inferLanguage(path) || "plaintext";
  const { leftVisibleLines, rightVisibleLines } = getVisibleLines(left, right, chunks);

  const [leftComponent, rightComponent] = await Promise.all([
    renderDiffSide(language, left.content, leftVisibleLines),
    renderDiffSide(language, right.content, rightVisibleLines),
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
