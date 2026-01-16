import { start } from "node:repl";
import type { Element, Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import {
  CONTEXT_LINES,
  createChangeMaps,
  expandLines,
  inferLanguage,
  pairLines,
} from "@/(repo)/[slug]/util";
import type { DiffChange, DiffHunk, RepositoryFile } from "@/lib/dto";
import { DiffLine } from "./diff-line";

export async function DiffSplit({
  left,
  right,
  hunks,
}: {
  left: RepositoryFile;
  right: RepositoryFile;
  hunks: DiffHunk[];
}) {
  const language = inferLanguage(left.path) || "plaintext";
  const { leftChangeMap, rightChangeMap } = createChangeMaps(hunks);
  const [leftSpans, rightSpans] = await Promise.all([
    renderSpans("left", language, leftChangeMap, left.content),
    renderSpans("right", language, rightChangeMap, right.content),
  ]);

  return (
    <div className="flex flex-col w-full">
      {hunks.map((hunk) => {
        const startingLine =
          hunk[0].lhs?.line_number || hunk[0].rhs?.line_number || 0;

        return (
          <Fragment
            key={`${hunk[0].lhs?.line_number}-${hunk[0].rhs?.line_number}`}
          >
            {startingLine > CONTEXT_LINES && (
              <span className="flex flex-row w-full h-20 items-center relative">
                <div className="w-1/2 border-border border-r h-full" />
                <div className="absolute left-0 right-0 flex items-center justify-center">
                  <div className="w-20 border-t border-border" />
                </div>
              </span>
            )}
            <DiffSection
              hunk={hunk}
              leftSpans={leftSpans}
              rightSpans={rightSpans}
            />
          </Fragment>
        );
      })}
    </div>
  );
}

async function renderSpans(
  side: "left" | "right",
  language: string,
  changeMap: Map<number, DiffChange[]>,
  content: string,
): Promise<Element[]> {
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
          node.type = "element";
          node.tagName = "diffline";

          node.properties["data-line-number"] = lineNumber;
          if (changeMap.has(lineNumber - 1)) {
            node.properties["data-line-type"] =
              side === "left" ? "removed" : "added";
          }
        },
      },
    ],
  });

  const root = hast as Root;
  const pre = root.children[0] as Element;
  const code = pre.children[0] as Element;

  return code.children.filter(
    (child): child is Element => child.type === "element",
  );
}

const sentinelSpan: Element = {
  type: "element",
  tagName: "diffline",
  properties: {
    class: "line w-full",
    "data-line-type": "sentinel",
  },
  children: [],
};

function DiffSection({
  hunk,
  leftSpans,
  rightSpans,
}: {
  hunk: DiffHunk;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const expandedLines = expandLines(
    pairLines(hunk),
    leftSpans.length,
    rightSpans.length,
  );

  const leftSpansChunk: Element[] = [];
  const rightSpansChunk: Element[] = [];
  for (const [left, right] of expandedLines) {
    leftSpansChunk.push(left !== null ? leftSpans[left] : sentinelSpan);
    rightSpansChunk.push(right !== null ? rightSpans[right] : sentinelSpan);
  }

  const container: Element = {
    type: "element",
    tagName: "div",
    properties: {
      className: "flex w-full",
    },
    children: [
      {
        type: "element",
        tagName: "pre",
        properties: {
          className:
            "flex flex-col w-1/2 overflow-auto scrollbar-none border-border border-r text-sm font-mono",
        },
        children: leftSpansChunk,
      },
      {
        type: "element",
        tagName: "pre",
        properties: {
          className:
            "flex flex-col w-1/2 overflow-auto scrollbar-none text-sm font-mono",
        },
        children: rightSpansChunk,
      },
    ],
  };

  return toJsxRuntime(container, {
    Fragment,
    jsx,
    jsxs,
    components: {
      diffline: (props) => <DiffLine {...props} />,
    },
  }) as JSX.Element;
}
