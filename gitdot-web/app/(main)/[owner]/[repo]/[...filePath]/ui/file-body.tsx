"use client";

import type { Element, Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { addClassToHast } from "shiki";
import type { LineSelection } from "../util";
import { FileBodyClient } from "./file-body-client";
import { FileLine } from "./file-line";

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

export function FileBody({
  selectedLines,
  hast,
}: {
  selectedLines: LineSelection | null;
  hast: Root;
}) {
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

  return (
    <div className="w-full text-sm">
      <FileBodyClient selectedLines={selectedLines}>{content}</FileBodyClient>
    </div>
  );
}
