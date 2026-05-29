"use client";

import type { DiffHunk } from "gitdot-dal/client";
import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment, useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { pluralize } from "@/util/string";
import { DiffLine } from "./diff-line";

export function DiffSplit({
  leftSpans,
  rightSpans,
  hunks,
}: {
  leftSpans: Element[];
  rightSpans: Element[];
  hunks: DiffHunk[];
}) {
  return (
    <div className="flex flex-col w-full">
      {hunks.map((hunk, index) => (
        <Fragment key={`${hunk.pairs[0][0]}-${hunk.pairs[0][1]}`}>
          {index > 0 && (
            <HiddenSection
              prev={hunks[index - 1]}
              next={hunk}
              leftSpans={leftSpans}
              rightSpans={rightSpans}
            />
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
  tagName: "diffline",
  properties: {
    class: "line w-full",
    "data-line-type": "sentinel",
  },
  children: [],
};

const withSide = (span: Element, side: "old" | "new"): Element => ({
  ...span,
  properties: { ...span.properties, "data-side": side },
});

const getSpanOrSentinel = (index: number | null, spans: Element[]) =>
  index !== null && index < spans.length ? spans[index] : sentinelSpan;

function DiffSection({
  hunk,
  leftSpans,
  rightSpans,
}: {
  hunk: DiffHunk;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const pairs = hunk.pairs;
  const leftSpansChunk = pairs.map(([left]) =>
    withSide(getSpanOrSentinel(left, leftSpans), "old"),
  );
  const rightSpansChunk = pairs.map(([, right]) =>
    withSide(getSpanOrSentinel(right, rightSpans), "new"),
  );

  return renderChunks(leftSpansChunk, rightSpansChunk);
}

function HiddenSection({
  prev,
  next,
  leftSpans,
  rightSpans,
}: {
  prev: DiffHunk;
  next: DiffHunk;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const [expanded, setExpanded] = useState(false);
  const last = prev.pairs.at(-1);
  const first = next.pairs[0];
  const prevL = last?.[0];
  const prevR = last?.[1];
  const nextL = first?.[0];
  if (prevL == null || prevR == null || nextL == null) return null;
  const count = nextL - prevL - 1;
  if (count <= 0) return null;

  if (!expanded) {
    return (
      <button
        type="button"
        onClick={() => setExpanded(true)}
        className="flex w-full h-6 items-center justify-center bg-sidebar border-y border-border font-mono text-xs text-muted-foreground hover:text-foreground transition-colors duration-200 cursor-pointer"
      >
        {pluralize(count, "line")}...
      </button>
    );
  }

  const leftChunk: Element[] = [];
  const rightChunk: Element[] = [];
  for (let i = 1; prevL + i < nextL; i++) {
    leftChunk.push(withSide(getSpanOrSentinel(prevL + i, leftSpans), "old"));
    rightChunk.push(withSide(getSpanOrSentinel(prevR + i, rightSpans), "new"));
  }

  return renderChunks(leftChunk, rightChunk);
}

function renderChunks(leftChunk: Element[], rightChunk: Element[]) {
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
        children: leftChunk,
      },
      {
        type: "element",
        tagName: "pre",
        properties: {
          className:
            "flex flex-col w-1/2 overflow-auto scrollbar-none text-sm font-mono",
        },
        children: rightChunk,
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
