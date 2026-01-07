import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import { getRepositoryCommits, getRepositoryFile } from "@/lib/dal";
import type { LineSelection } from "../util";
import { inferLanguage } from "../util";
import { FileLine } from "./file-line";
import { FileViewerClient } from "./file-viewer-client";

export async function FileViewer({
  repo,
  filePath,
  selectedLines,
}: {
  repo: string;
  filePath: string;
  selectedLines: LineSelection | null;
}) {
  const file = await getRepositoryFile("bkdevs", repo, { path: filePath });
  if (!file) {
    return <div>File not found.</div>;
  }

  const commits = await getRepositoryCommits("bkdevs", repo);
  console.log(commits);

  const hast = await codeToHast(file.content, {
    lang: inferLanguage(filePath) ?? "plaintext",
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

  const content = toJsxRuntime(hast, {
    Fragment,
    jsx,
    jsxs,
    components: {
      fileline: (props) => <FileLine {...props} />,
    },
  }) as JSX.Element;

  return (
    <div className="w-full h-full overflow-auto text-sm">
      <FileViewerClient selectedLines={selectedLines}>
        {content}
      </FileViewerClient>
    </div>
  );
}
