"use client";

import type { DiffHunk } from "gitdot-dal/client";
import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment, useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { pluralize } from "@/util/string";
import { DiffLine } from "./diff-line";

export function DiffUnilateral({
  spans,
  hunks,
  side,
}: {
  spans: Element[];
  hunks: DiffHunk[];
  side: "left" | "right";
}) {
  return (
    <div className="flex flex-col w-full">
      {hunks.map((hunk, index) => (
        <Fragment key={`${hunk.pairs[0][0]}-${hunk.pairs[0][1]}`}>
          {index > 0 && (
            <HiddenSection
              prev={hunks[index - 1]}
              next={hunk}
              spans={spans}
              side={side}
            />
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

function withSide(span: Element, side: "left" | "right"): Element {
  return {
    ...span,
    properties: {
      ...span.properties,
      "data-side": side === "left" ? "old" : "new",
    },
  };
}

function DiffSection({
  hunk,
  spans,
  side,
}: {
  hunk: DiffHunk;
  spans: Element[];
  side: "left" | "right";
}) {
  const outputSpans: Element[] = [];
  for (const [L, R] of hunk.pairs) {
    const idx = side === "left" ? L : R;
    if (idx === null) continue;
    const span = idx < spans.length ? spans[idx] : sentinelSpan;
    outputSpans.push(withSide(span, side));
  }

  return renderChunk(outputSpans);
}

function HiddenSection({
  prev,
  next,
  spans,
  side,
}: {
  prev: DiffHunk;
  next: DiffHunk;
  spans: Element[];
  side: "left" | "right";
}) {
  const [expanded, setExpanded] = useState(false);
  const sideIdx = side === "left" ? 0 : 1;
  const prevIdx = prev.pairs.at(-1)?.[sideIdx];
  const nextIdx = next.pairs[0]?.[sideIdx];
  if (prevIdx == null || nextIdx == null) return null;
  const count = nextIdx - prevIdx - 1;
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

  const outputSpans: Element[] = [];
  for (let i = prevIdx + 1; i < nextIdx && i < spans.length; i++) {
    outputSpans.push(withSide(spans[i], side));
  }

  return renderChunk(outputSpans);
}

function renderChunk(outputSpans: Element[]) {
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
