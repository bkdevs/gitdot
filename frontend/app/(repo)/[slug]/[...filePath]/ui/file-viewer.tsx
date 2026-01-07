import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast, codeToHtml, codeToTokens } from "shiki";
import { getRepositoryFile } from "@/lib/dal";
import { inferLanguage } from "@/util";
import { FileLine } from "./file-line";

export async function FileViewer({
  repo,
  filePath,
}: {
  repo: string;
  filePath: string;
}) {
  const file = await getRepositoryFile("bkdevs", repo, { path: filePath });
  if (!file) {
    return <div>File not found.</div>;
  }

  const hast = await codeToHast(file.content, {
    lang: inferLanguage(filePath) ?? "plaintext",
    theme: "vitesse-light",
    transformers: [
      {
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
    <div className="w-full h-full overflow-auto px-2 text-sm">{content}</div>
  );
}
