import type { DiffHunkResource } from "gitdot-api";
import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { expandLines, pairLines } from "@/(main)/[owner]/[repo]/util";
import { DiffLine } from "./diff-line";

export function DiffUnilateral({
  spans,
  hunks,
  side,
}: {
  spans: Element[];
  hunks: DiffHunkResource[];
  side: "left" | "right";
}) {
  return (
    <div className="flex flex-col w-full">
      {hunks.map((hunk, index) => (
        <Fragment
          key={`${hunk[0].lhs?.line_number}-${hunk[0].rhs?.line_number}`}
        >
          {index > 0 && (
            <span className="flex w-full h-20 items-center justify-center">
              <div className="w-20 border-t border-border" />
            </span>
          )}
          <DiffSection hunk={hunk} spans={spans} side={side} />
        </Fragment>
      ))}
    </div>
  );
}

const sentinelSpan: Element = {
  type: "element",
  tagName: "diffline",
  properties: { "data-line-type": "sentinel", "data-line-number": 0 },
  children: [],
};

function DiffSection({
  hunk,
  spans,
  side,
}: {
  hunk: DiffHunkResource;
  spans: Element[];
  side: "left" | "right";
}) {
  const pairs = expandLines(pairLines(hunk), spans.length, spans.length);

  const outputSpans: Element[] = [];
  for (const [L, R] of pairs) {
    const idx = side === "left" ? L : R;
    if (idx === null) continue;
    outputSpans.push(idx < spans.length ? spans[idx] : sentinelSpan);
  }

  const container: Element = {
    type: "element",
    tagName: "pre",
    properties: {
      className:
        "flex flex-col w-full overflow-auto scrollbar-none text-sm font-mono",
    },
    children: outputSpans,
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
