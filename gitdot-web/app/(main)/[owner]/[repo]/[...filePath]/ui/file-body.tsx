import type { RepositoryFileResource } from "gitdot-api";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { fileToHast } from "@/(main)/[owner]/[repo]/util";
import type { LineSelection } from "../util";
import { FileBodyClient } from "./file-body-client";
import { FileLine } from "./file-line";

export async function FileBody({
  file,
  selectedLines,
}: {
  file: RepositoryFileResource;
  selectedLines: LineSelection | null;
}) {
  const hast = await fileToHast(file, "vitesse-light", [
    {
      pre(node) {
        this.addClassToHast(node, "outline-none");
      },
      line(node, line) {
        node.tagName = "fileline";
        node.properties["data-line-number"] = line;
      },
    },
  ]);

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
