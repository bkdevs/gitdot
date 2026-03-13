import type { RepositoryFileResource } from "gitdot-api";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment, use } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import type { ShikiTransformer } from "shiki";
import { fileToHast } from "@/(main)/[owner]/[repo]/util";
import type { LineSelection } from "../util";
import { FileBodyClient } from "./file-body-client";
import { FileLine } from "./file-line";

const FILE_BODY_TRANSFORMERS: ShikiTransformer[] = [
  {
    pre(node) {
      this.addClassToHast(node, "outline-none");
    },
    line(node, line) {
      node.tagName = "fileline";
      node.properties["data-line-number"] = line;
    },
  },
];

export function FileBody({
  file,
  selectedLines,
}: {
  file: RepositoryFileResource;
  selectedLines: LineSelection | null;
}) {
  const hast = use(fileToHast(file, "vitesse-light", FILE_BODY_TRANSFORMERS));

  const content = toJsxRuntime(hast, {
    Fragment,
    jsx,
    jsxs,
    components: {
      fileline: (props) => <FileLine {...props} />,
    },
  }) as JSX.Element;

  return (
    <div className="w-full text-sm">
      <FileBodyClient selectedLines={selectedLines}>{content}</FileBodyClient>
    </div>
  );
}
