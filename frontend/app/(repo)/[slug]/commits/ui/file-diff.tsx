import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import type { RepositoryFile } from "@/lib/dto";
import { inferLanguage } from "@/(repo)/[slug]/util";
import { RepositoryFileDiff } from "@/lib/dto";
import { FileLine } from "../../[...filePath]/ui/file-line";
import { FileViewerClient } from "../../[...filePath]/ui/file-viewer-client";

async function renderFile(file: RepositoryFile | undefined) {
  if (!file?.content) {
    return null;
  }

  const hast = await codeToHast(file.content, {
    lang: inferLanguage(file.path) ?? "plaintext",
    theme: "vitesse-light",
    transformers: [
      {
        pre(node) {
          this.addClassToHast(node, "outline-none");
        },
        line(node, line) {
          node.tagName = "fileline";
          node.properties["data-line-number"] = line;
        },
      },
    ],
  });

  return toJsxRuntime(hast, {
    Fragment,
    jsx,
    jsxs,
    components: {
      fileline: (props) => <FileLine {...props} />,
    },
  }) as JSX.Element;
}

export async function FileDiff({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right } = diff;
  const path = left?.path || right?.path;

  const [leftContent, rightContent] = await Promise.all([
    renderFile(left),
    renderFile(right)
  ]);

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
        {path}
      </div>
      <div className="flex flex-row w-full">
        <div className="flex flex-col w-1/2 text-sm font-mono overflow-x-auto">
          <FileViewerClient selectedLines={null}>
            {leftContent}
          </FileViewerClient>
        </div>
        <div className="flex flex=col w-1/2 text-sm font-mono overflow-x-auto border-border border-l">
          <FileViewerClient selectedLines={null}>
            {rightContent}
          </FileViewerClient>
        </div>
      </div>
    </div>
  )
}
