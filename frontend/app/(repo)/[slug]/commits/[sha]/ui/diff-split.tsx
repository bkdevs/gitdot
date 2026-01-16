import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import {
  expandLines,
  inferLanguage,
  pairLines,
  renderSpans,
  sentinelSpan,
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
  const language = inferLanguage(left.path) || "plaintext";
  const [leftSpans, rightSpans] = await Promise.all([
    renderSpans(language, left.content),
    renderSpans(language, right.content),
  ]);

  return (
    <div className="flex flex-col w-full gap-8">
      {hunks.map((hunk) => (
        <DiffSection
          key={`${hunk[0].lhs?.line_number}-${hunk[0].rhs?.line_number}`}
          hunk={hunk}
          leftSpans={leftSpans}
          rightSpans={rightSpans}
        />
      ))}
    </div>
  );
}

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
      className: "flex w-full border-t border-b border-border",
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
