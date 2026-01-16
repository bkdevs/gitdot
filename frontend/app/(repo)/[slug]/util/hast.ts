import type { Element, Root } from "hast";
import { codeToHast } from "shiki";

/**
 * given a language and a file as a string, returns a list of highlighted <span> elements
 *
 * note that these spans are all self-contained, shiki uses in-line styles so we can post-process
 * and use them directly in resulting JSX
 */
export async function renderSpans(
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

export const sentinelSpan: Element = {
  type: "element",
  tagName: "diffline",
  properties: {
    class: "line w-full",
    "data-line-type": "sentinel",
  },
  children: [],
};
