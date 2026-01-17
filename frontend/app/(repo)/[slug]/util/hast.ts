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
          if (changes && changes.length > 0) {
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

function highlightChanges(
  side: "left" | "right",
  lineNode: Element,
  changes: DiffChange[],
): void {
  if (changes.length === 0) return;

  let charOffset = 0;

  for (const child of lineNode.children) {
    if (child.type !== "element") {
      if (child.type === "text") {
        charOffset += child.value.length;
      }
      continue;
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
  if (node.type === "text") {
    return node.value.length;
  }
  if (node.type === "element") {
    let length = 0;
    for (const child of node.children) {
      length += getSpanLength(child);
    }
    return length;
  }
  return 0;
}
