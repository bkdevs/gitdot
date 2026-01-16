import type { Element, ElementContent, Root } from "hast";
import { codeToHast } from "shiki";
import type { DiffChange } from "@/lib/dto";

export async function renderSpans(
  side: "left" | "right",
  language: string,
  changeMap: Map<number, DiffChange[]>,
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
          this.addClassToHast(node, "flex flex-col");
        },
        line(node, lineNumber) {
          node.type = "element";
          node.tagName = "diffline";

          node.properties["data-line-number"] = lineNumber;
          const changes = changeMap.get(lineNumber - 1);
          if (changes && changes.length > 0) {
            node.properties["data-line-type"] =
              side === "left" ? "removed" : "added";
            highlightChanges(node, changes);
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

function highlightChanges(lineNode: Element, changes: DiffChange[]): void {
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
        const existingStyle = child.properties.style;
        if (typeof existingStyle === "string") {
          const trimmed = existingStyle.trim();
          const sep = trimmed && !trimmed.endsWith(";") ? ";" : "";
          child.properties.style = `${trimmed}${sep}font-weight:bold`;
        } else {
          child.properties.style = "font-weight:bold";
        }
        break;
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
