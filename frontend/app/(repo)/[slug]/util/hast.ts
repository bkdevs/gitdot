import type { Element, ElementContent, Root } from "hast";
import { addClassToHast, getSingletonHighlighter } from "shiki";
import type { DiffChange } from "@/lib/dto";
import { loadGitdotLight } from "@/lib/shiki";

export async function renderSpans(
  side: "left" | "right",
  language: string,
  changeMap: Map<number, DiffChange[]>,
  content: string,
): Promise<Element[]> {
  await loadGitdotLight();
  const highlighter = await getSingletonHighlighter();
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
            highlightWords(side, node, changes);
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

/**
 * apply word-level highlighting from difftastic output
 *
 * in general, we want our spans to be _wider_ than the changes, and then we create subspans within each span
 * with highlighting applied
 */
export function highlightWords(
  side: "left" | "right",
  lineNode: Element,
  changes: DiffChange[],
): void {
  const processedChanges = processChanges(changes);
  const colorClass = side === "left" ? "text-red-600!" : "text-green-600!";

  let charOffset = 0;
  for (const child of lineNode.children) {
    if (child.type !== "element") {
      throw new Error("Unexpected non-element child");
    }

    const spanLength = getSpanLength(child);
    const spanStart = charOffset;
    const spanEnd = charOffset + spanLength;

    const changesInSpan = processedChanges.filter(
      (change) => change.start >= spanStart && change.end <= spanEnd,
    );

    if (changesInSpan.length > 0) {
      const coversEntireSpan = changesInSpan.some(
        (change) => change.start === spanStart && change.end === spanEnd,
      );

      if (coversEntireSpan) {
        addClassToHast(child, colorClass);
      } else {
        splitSpan(child, spanStart, changesInSpan, colorClass);
      }
    }

    charOffset = spanEnd;
  }
}

/**
 * occasionally difft will return string changes as contiugous chunks,
 * e.g., "a string here" -> is one change
 *
 * that doesn't fit well into our spans, as we split out quotes as separate spans:
 * <span>"</span><span>a string here</span><span>"</span>
 *
 * so we process the changes to split out quotes
 */
function processChanges(changes: DiffChange[]): DiffChange[] {
  const result: DiffChange[] = [];

  for (const change of changes) {
    if (change.highlight !== "string") {
      result.push(change);
      continue;
    }

    const content = change.content;
    const quote = content[0];

    if (
      (quote === '"' || quote === "'" || quote === "`") &&
      content.length >= 2 &&
      content[content.length - 1] === quote
    ) {
      result.push({
        start: change.start,
        end: change.start + 1,
        content: quote,
        highlight: "string",
      });

      if (content.length > 2) {
        result.push({
          start: change.start + 1,
          end: change.end - 1,
          content: content.slice(1, -1),
          highlight: "string",
        });
      }

      result.push({
        start: change.end - 1,
        end: change.end,
        content: quote,
        highlight: "string",
      });
    } else {
      result.push(change);
    }
  }

  return result;
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

function getSpanText(node: Element): string {
  const child = node.children[0];
  if (child.type !== "text") {
    throw new Error("Span must have one text child");
  }
  return child.value;
}

function splitSpan(
  span: Element,
  spanStart: number,
  changes: DiffChange[],
  colorClass: string,
): void {
  const text = getSpanText(span);
  const sortedChanges = [...changes].sort((a, b) => a.start - b.start);

  const newChildren: ElementContent[] = [];
  let currentPos = spanStart;

  for (const change of sortedChanges) {
    if (change.start > currentPos) {
      const beforeText = text.slice(
        currentPos - spanStart,
        change.start - spanStart,
      );
      newChildren.push({ type: "text", value: beforeText });
    }

    const changeText = text.slice(
      change.start - spanStart,
      change.end - spanStart,
    );
    newChildren.push({
      type: "element",
      tagName: "span",
      properties: { class: [colorClass] },
      children: [{ type: "text", value: changeText }],
    });

    currentPos = change.end;
  }

  if (currentPos < spanStart + text.length) {
    const afterText = text.slice(currentPos - spanStart);
    newChildren.push({ type: "text", value: afterText });
  }

  span.children = newChildren;
}
