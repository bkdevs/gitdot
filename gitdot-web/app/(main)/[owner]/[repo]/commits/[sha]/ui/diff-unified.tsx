import type { DiffHunkResource } from "gitdot-api";
import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import {
  expandLines,
  type LinePair,
  pairLines,
} from "@/(main)/[owner]/[repo]/util";
import { cn } from "@/util";

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
      {hunks.map((hunk, index) => (
        <Fragment
          key={`${hunk[0].lhs?.line_number}-${hunk[0].rhs?.line_number}`}
        >
          {index > 0 && (
            <span className="flex w-full h-20 items-center justify-center">
              <div className="w-20 border-t border-border" />
            </span>
          )}
          <DiffSection
            hunk={hunk}
            leftSpans={leftSpans}
            rightSpans={rightSpans}
          />
        </Fragment>
      ))}
    </div>
  );
}

const sentinelSpan: Element = {
  type: "element",
  tagName: "difflineunified",
  properties: { "data-line-type": "normal" },
  children: [],
};

function lineType(spans: Element[], idx: number): string {
  return String(spans[idx]?.properties?.["data-line-type"] ?? "normal");
}

function makeSpan(
  span: Element,
  leftNum: number | undefined,
  rightNum: number | undefined,
): Element {
  return {
    ...span,
    tagName: "difflineunified",
    properties: {
      ...span.properties,
      "data-left-line-number": leftNum,
      "data-right-line-number": rightNum,
    },
  };
}

function DiffSection({
  hunk,
  leftSpans,
  rightSpans,
}: {
  hunk: DiffHunkResource;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const pairs = expandLines(
    pairLines(hunk),
    leftSpans.length,
    rightSpans.length,
  );

  const isChanged = ([L, R]: LinePair): boolean =>
    L === null ||
    R === null ||
    lineType(leftSpans, L) !== "normal" ||
    lineType(rightSpans, R) !== "normal";

  const firstChangeIdx = pairs.findIndex(isChanged);
  if (firstChangeIdx < 0) return null;
  const lastChangeIdx = pairs.findLastIndex(isChanged);

  const before = pairs.slice(0, firstChangeIdx);
  const modified = pairs.slice(firstChangeIdx, lastChangeIdx + 1);
  const after = pairs.slice(lastChangeIdx + 1);

  const outputSpans: Element[] = [];

  // output unchanged lines for context
  for (const [L, R] of before) {
    if (L !== null && R !== null)
      outputSpans.push(
        makeSpan(
          L < leftSpans.length ? leftSpans[L] : sentinelSpan,
          L + 1,
          R + 1,
        ),
      );
  }

  // output removed lines of the patch first
  for (const [L] of modified) {
    if (L !== null && lineType(leftSpans, L) === "removed")
      outputSpans.push(makeSpan(leftSpans[L], L + 1, undefined));
  }

  // output added lines afterwards
  for (const [L, R] of modified) {
    if (R !== null && (L === null || lineType(rightSpans, R) === "added"))
      outputSpans.push(
        makeSpan(
          R < rightSpans.length ? rightSpans[R] : sentinelSpan,
          undefined,
          R + 1,
        ),
      );
  }

  // output unchanged lines for context
  for (const [L, R] of after) {
    if (L !== null && R !== null)
      outputSpans.push(
        makeSpan(
          L < leftSpans.length ? leftSpans[L] : sentinelSpan,
          L + 1,
          R + 1,
        ),
      );
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
      difflineunified: (props) => <DiffLineUnified {...props} />,
    },
  }) as JSX.Element;
}

function DiffLineUnified({
  children,
  "data-left-line-number": leftNum,
  "data-right-line-number": rightNum,
  "data-line-type": lineType,
}: {
  children: React.ReactNode;
  "data-left-line-number": number | undefined;
  "data-right-line-number": number | undefined;
  "data-line-type": "normal" | "added" | "removed";
}) {
  return (
    <span
      className={cn(
        "diff-line",
        "flex items-center w-full",
        lineType === "added" && "bg-diff-green",
        lineType === "removed" && "bg-diff-red",
      )}
      data-left-line-number={leftNum}
      data-right-line-number={rightNum}
      data-line-type={lineType}
    >
      <span className="w-7 text-right shrink-0 pr-1 text-xs leading-5 text-primary/30 select-none">
        {leftNum ?? ""}
      </span>
      <span className="w-7 text-right shrink-0 pr-1 mr-1 text-xs leading-5 text-primary/30 select-none">
        {rightNum ?? ""}
      </span>
      {children}
    </span>
  );
}
