import type { Element, ElementContent, Root } from "hast";
import { addClassToHast } from "shiki";
import type { DiffChange } from "@/lib/dto";
import { highlighter } from "@/lib/shiki";

export async function renderSpans(
  side: "left" | "right",
  language: string,
  changeMap: Map<number, DiffChange[]>,
  content: string,
): Promise<Element[]> {
  const hast = highlighter.codeToHast(content, {
    lang: language,
    theme: "gitdot-light",
    transformers: [
      {
        pre(node) {
          this.addClassToHast(node, "outline-none");
        },
        code(node) {
          this.addClassToHast(node, "flex flex-col");
        },
        line(node, lineNumber) {
          node.type = "element";
          node.tagName = "diffline";
          node.properties["data-line-number"] = lineNumber;

          const changes = changeMap.get(lineNumber - 1);
          if (changes) {
            highlightChanges(side, node, changes);
          }
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

export function highlightChanges(
  side: "left" | "right",
  lineNode: Element,
  changes: DiffChange[],
): void {
  let charOffset = 0;

  for (const child of lineNode.children) {
    if (child.type !== "element") {
      throw new Error("Unexpected non-element child");
    }

    const spanLength = getSpanLength(child);
    const spanStart = charOffset;
    const spanEnd = charOffset + spanLength;

    for (const change of changes) {
      if (change.start >= spanStart && change.end <= spanEnd) {
        addClassToHast(
          child,
          side === "left" ? "text-red-600!" : "text-green-600!",
        );
      }
    }

    charOffset = spanEnd;
  }
}

function getSpanLength(node: ElementContent): number {
  if (node.type !== "element" || node.children.length !== 1) {
    throw new Error("Span must have one child");
  }
  const child = node.children[0];
  if (child.type !== "text") {
    throw new Error("Span must have one text child");
  }
  return child.value.length;
}
