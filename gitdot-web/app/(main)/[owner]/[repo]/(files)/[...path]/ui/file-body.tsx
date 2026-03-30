"use client";

import type { Element, Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { addClassToHast } from "shiki";
import { cn } from "@/util";
import { useFileViewerContext } from "./file-viewer-context";

export function FileBody() {
  const { hast } = useFileViewerContext();

  const content = toJsxRuntime(
    applyFileBodyTransformers(structuredClone(hast)),
    {
      Fragment,
      jsx,
      jsxs,
      components: {
        fileline: (props) => <FileLine {...props} />,
      },
    },
  ) as JSX.Element;

  return <div className="w-full text-sm">{content}</div>;
}

function applyFileBodyTransformers(hast: Root): Root {
  const pre = hast.children[0] as Element;
  addClassToHast(pre, "outline-none");

  const code = pre.children[0] as Element;
  let lineIndex = 0;
  for (const child of code.children) {
    if (child.type === "element") {
      child.tagName = "fileline";
      child.properties["data-line-number"] = lineIndex + 1;
      lineIndex++;
    }
  }
  return hast;
}

function FileLine({
  children,
  "data-line-number": lineNumber,
  style,
}: {
  children: React.ReactNode;
  "data-line-number": number;
  style?: React.CSSProperties;
}) {
  const { isLineSelected, handleLineMouseDown, handleLineMouseEnter } =
    useFileViewerContext();

  const isSelected = isLineSelected(lineNumber);

  return (
    <span
      className={cn("inline-flex w-full", isSelected && "bg-accent/60")}
      style={style}
      onMouseEnter={() => handleLineMouseEnter(lineNumber)}
    >
      <button
        type="button"
        className="w-9 text-right shrink-0 pr-1.5 mr-1 text-primary/60 select-none outline-none cursor-pointer"
        onMouseDown={() => handleLineMouseDown(lineNumber)}
      >
        {lineNumber}
      </button>
      {children}
    </span>
  );
}
