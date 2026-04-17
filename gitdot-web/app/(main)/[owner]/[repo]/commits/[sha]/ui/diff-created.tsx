import type { Element } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { DiffLine } from "./diff-line";

export function DiffCreated({ spans }: { spans: Element[] }) {
  const container: Element = {
    type: "element",
    tagName: "pre",
    properties: { className: "flex flex-col outline-none" },
    children: spans,
  };
  const content = toJsxRuntime(container, {
    Fragment,
    jsx,
    jsxs,
    components: {
      diffline: (props) => <DiffLine {...props} />,
    },
  }) as JSX.Element;

  return (
    <div className="w-full h-full overflow-auto text-sm scrollbar-none">
      {content}
    </div>
  );
}
