import type { Element, Root } from "hast";
import { codeToHast } from "shiki";
import { inferLanguage } from "@/(repo)/[slug]/util";
import type { RepositoryFileDiff } from "@/lib/dto";
import { DiffChunk } from "./diff-chunk";

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
  const { left, right, chunks } = diff;
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
  if (!chunks || chunks.length === 0) {
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

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
        {path}
      </div>
      <div className="flex flex-col w-full gap-8">
        {chunks.map((chunk, index) => (
          <DiffChunk
          key={index}
          chunk={chunk}
          leftSpans={leftSpans}
          rightSpans={rightSpans}
          />
        ))}
      </div>
    </div>
  );
}
