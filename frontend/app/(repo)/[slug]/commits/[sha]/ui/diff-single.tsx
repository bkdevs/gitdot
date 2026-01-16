import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import { inferLanguage } from "@/(repo)/[slug]/util";
import type { RepositoryFile } from "@/lib/dto";

import { DiffLine } from "./diff-line";

export async function DiffSingle({
  file,
  side,
}: {
  file: RepositoryFile;
  side: "left" | "right";
}) {
  const hast = await codeToHast(file.content, {
    lang: inferLanguage(file.path) ?? "plaintext",
    theme: "vitesse-light",
    transformers: [
      {
        pre(node) {
          this.addClassToHast(node, "outline-none");
        },
        line(node, line) {
          node.tagName = "diffline";
          node.properties["data-line-number"] = line;

          // hues taken from tailwind red-50 and green-50, then adjusting chroma down and brightness to be equal =
          node.properties["data-bg-color"] =
            side === "left"
              ? "oklch(98.5% 0.01 17.38)"
              : "oklch(98.5% 0.01 155.826)";
        },
      },
    ],
  });
  const content = toJsxRuntime(hast, {
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
