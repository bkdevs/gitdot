import { fileToHast } from "@/(main)/[owner]/[repo]/util";
import type { RepositoryFile } from "@/lib/dto";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import type { LineSelection } from "../util";
import { FileLine } from "./file-line";
import { FileViewerClient } from "./file-viewer-client";

export async function FileBody({
  file,
  selectedLines,
}: {
  file: RepositoryFile;
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
      <FileViewerClient selectedLines={selectedLines}>
        {content}
      </FileViewerClient>
    </div>
  );
}
