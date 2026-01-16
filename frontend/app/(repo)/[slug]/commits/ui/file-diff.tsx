import type { Element, Root } from "hast";
import { codeToHast } from "shiki";
import { inferLanguage } from "@/(repo)/[slug]/util";
import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffSection } from "./diff-section";

async function renderSpans(
  language: string,
  content: string,
): Promise<Element[]> {
  const hast = await codeToHast(content, {
    lang: language,
    theme: "vitesse-light",
    transformers: [
      {
        pre(node) {
          this.addClassToHast(node, "outline-none");
        },
        code(node) {
          // required as shiki by default renders code as a line
          this.addClassToHast(node, "flex flex-col");
        },
        line(node, lineNumber) {
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;
          node.type = "element";
        },
      },
    ],
  });

  const root = hast as Root;
  const pre = root.children[0] as Element;
  const code = pre.children[0] as Element;

  return code.children.filter(
    (child): child is Element => child.type === "element",
  );
}

export async function FileDiff({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, hunks } = diff;
  const path = left?.path || right?.path;
  if (!path) {
    throw new Error("File path or chunks are missing");
  }
  if (!left?.content || !right?.content) {
    return (
      <div>
        one of the two files are missing, this is unimplemented as of now,
        should show a single file view
      </div>
    );
  }
  if (!hunks || hunks.length === 0) {
    return (
      <div className="w-full">
        TK, this gets returned when files are deleted, created, or renamed
      </div>
    );
  }

  const language = inferLanguage(path) || "plaintext";
  const [leftSpans, rightSpans] = await Promise.all([
    renderSpans(language, left.content),
    renderSpans(language, right.content),
  ]);

  console.log("file lengths");
  console.log(left.content.split("\n").length);
  console.log(right.content.split("\n").length);

  console.log("span lengths");
  console.log(leftSpans.length);
  console.log(rightSpans.length);

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
        {path}
      </div>
      <div className="flex flex-col w-full gap-8">
        {hunks.map((hunk, index) => (
          <DiffSection
            key={index}
            hunk={hunk}
            leftSpans={leftSpans}
            rightSpans={rightSpans}
          />
        ))}
      </div>
    </div>
  );
}
