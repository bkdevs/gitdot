import type { DiffHunkResource } from "gitdot-api";
import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { expandLines, pairLines } from "@/(main)/[owner]/[repo]/util";
import { DiffLine } from "./diff-line";

export function DiffUnified({
  leftSpans,
  rightSpans,
  hunks,
}: {
  leftSpans: Element[];
  rightSpans: Element[];
  hunks: DiffHunkResource[];
}) {
  return (
    <div className="flex flex-col w-full">
      {hunks.map((hunk, index) => {
        return (
          <Fragment
            key={`${hunk[0].lhs?.line_number}-${hunk[0].rhs?.line_number}`}
          >
            {index > 0 && (
              <span className="flex w-full h-20 items-center justify-center">
                <div className="w-20 border-t border-border" />
              </span>
            )}
            <UnifiedSection
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

function UnifiedSection({
  hunk,
  leftSpans,
  rightSpans,
}: {
  hunk: DiffHunkResource;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const expandedLines = expandLines(
    pairLines(hunk),
    leftSpans.length,
    rightSpans.length,
  );

  const withLineType = (
    span: Element,
    lineType: "normal" | "added" | "removed",
  ): Element => ({
    ...span,
    properties: { ...span.properties, "data-line-type": lineType },
  });

  const getSpan = (index: number | null, spans: Element[]): Element | null =>
    index !== null && index < spans.length ? spans[index] : null;

  const spans: Element[] = [];
  for (const [left, right] of expandedLines) {
    if (left !== null && right !== null) {
      const span = getSpan(left, leftSpans);
      if (span) spans.push(withLineType(span, "normal"));
    } else if (left !== null) {
      const span = getSpan(left, leftSpans);
      if (span) spans.push(withLineType(span, "removed"));
    } else if (right !== null) {
      const span = getSpan(right, rightSpans);
      if (span) spans.push(withLineType(span, "added"));
    }
  }

  const container: Element = {
    type: "element",
    tagName: "pre",
    properties: {
      className:
        "flex flex-col w-full overflow-auto scrollbar-none text-sm font-mono",
    },
    children: spans,
  };

  return toJsxRuntime(container, {
    Fragment,
    jsx,
    jsxs,
    components: { diffline: (props) => <DiffLine {...props} /> },
  }) as JSX.Element;
}
