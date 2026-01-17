import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import {
  CONTEXT_LINES,
  createChangeMaps,
  expandLines,
  inferLanguage,
  pairLines,
  renderSpans,
} from "@/(repo)/[slug]/util";
import type { DiffHunk, RepositoryFile } from "@/lib/dto";
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
  const { leftChangeMap, rightChangeMap } = createChangeMaps(hunks);
  const [leftSpans, rightSpans] = await Promise.all([
    renderSpans("left", left, leftChangeMap),
    renderSpans("right", right, rightChangeMap),
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
