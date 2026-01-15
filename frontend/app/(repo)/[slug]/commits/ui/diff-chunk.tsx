import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { pairLines } from "@/(repo)/[slug]/util";
import type { DiffChunk } from "@/lib/dto";
import { DiffLine } from "./diff-line";

const sentinelSpan: Element = {
  type: "element",
  tagName: "diffline",
  properties: {
    class: "line w-full",
    "data-line-type": "sentinel",
  },
  children: [],
};

export function DiffChunk({
  chunk,
  leftSpans,
  rightSpans,
}: {
  chunk: DiffChunk;
  leftSpans: Element[];
  rightSpans: Element[];
}) {
  const leftSpansChunk: Element[] = [];
  const rightSpansChunk: Element[] = [];
  const pairedLines = pairLines(chunk);

  for (const [left, right] of pairedLines) {
    leftSpansChunk.push(left !== null ? leftSpans[left] : sentinelSpan);
    rightSpansChunk.push(right !== null ? rightSpans[right] : sentinelSpan);
  }

  const container: Element = {
    type: "element",
    tagName: "div",
    properties: {
      className: "flex w-full border-t border-b border-border",
    },
    children: [
      {
        type: "element",
        tagName: "pre",
        properties: {
          className:
            "flex flex-col w-1/2 overflow-auto border-border border-r text-sm font-mono",
        },
        children: leftSpansChunk,
      },
      {
        type: "element",
        tagName: "pre",
        properties: {
          className: "flex flex-col overflow-auto w-1/2 text-sm font-mono",
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
