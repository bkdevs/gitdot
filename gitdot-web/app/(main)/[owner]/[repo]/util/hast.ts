import type { DiffChangeResource, RepositoryFileResource } from "gitdot-api";
import type { Element, ElementContent, Root } from "hast";
import {
  addClassToHast,
  type BundledLanguage,
  getSingletonHighlighter,
  type Highlighter,
  type ShikiTransformer,
} from "shiki";
import { inferLanguage } from "./language";

/**
 * shiki short hands (e.g., shiki.codeToHast) internally manages a singleton instance and lazily loads themes and bundles as required
 *
 * that is all what we'd like to leverage as well, with the exception that we want to inject our own custom themes when requested
 * so we consolidate on the following API for our own internal usage
 */
async function getHighlighter(
  lang: BundledLanguage | undefined,
  theme: "vitesse-light" | "gitdot-light",
): Promise<Highlighter> {
  const highlighter = await getSingletonHighlighter();

  if (!highlighter.getLoadedThemes().includes(theme)) {
    if (theme === "gitdot-light") {
      const gitdotLight = (await import("@/themes/gitdot-light")).default;
      await highlighter.loadTheme(gitdotLight);
    } else if (theme === "vitesse-light") {
      const vitesseLight = (await import("@shikijs/themes/vitesse-light"))
        .default;
      await highlighter.loadTheme(vitesseLight);
    }
  }

  if (lang && !highlighter.getLoadedLanguages().includes(lang)) {
    await highlighter.loadLanguage(lang);
  }
  return highlighter;
}

export async function fileToHast(
  content: string,
  lang: BundledLanguage | undefined,
  theme: "vitesse-light" | "gitdot-light",
  transformers: ShikiTransformer[],
) {
  const highlighter = await getHighlighter(lang, theme);

  return highlighter.codeToHast(content, {
    lang: lang ?? "plaintext",
    theme,
    transformers,
  });
}

export async function renderFileToHtml(
  file: RepositoryFileResource,
  theme: "vitesse-light" | "gitdot-light",
): Promise<string> {
  const lang = inferLanguage(file.path);
  const highlighter = await getHighlighter(lang, theme);

  return highlighter.codeToHtml(file.content, {
    lang: lang ?? "plaintext",
    theme,
  });
}

export async function renderSpans(
  side: "left" | "right",
  content: string,
  lang: BundledLanguage | undefined,
  changeMap: Map<number, DiffChangeResource[]>,
): Promise<Element[]> {
  const hast = await fileToHast(content, lang, "gitdot-light", [
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
          node.properties["data-line-type"] =
            side === "left" ? "removed" : "added";
          highlightWords(side, node, changes);
        } else {
          node.properties["data-line-type"] = "normal";
        }
      },
    },
  ]);

  const root = hast as Root;
  const pre = root.children[0] as Element;
  const code = pre.children[0] as Element;

  const lines = code.children.filter(
    (child): child is Element => child.type === "element",
  );
  for (const line of lines) {
    splitLineByWhitespace(line);
  }

  return lines;
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
  changes: DiffChangeResource[],
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
        splitSpanByWord(child, spanStart, changesInSpan, colorClass);
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
function processChanges(changes: DiffChangeResource[]): DiffChangeResource[] {
  const result: DiffChangeResource[] = [];

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

function splitSpanByWord(
  span: Element,
  spanStart: number,
  changes: DiffChangeResource[],
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

function getTextLength(node: ElementContent): number {
  if (node.type === "text") return node.value.length;
  if (node.type === "element") {
    return node.children.reduce((sum, c) => sum + getTextLength(c), 0);
  }
  return 0;
}

function splitLineByWhitespace(lineNode: Element): void {
  const newChildren: ElementContent[] = [];
  let charOffset = 0;

  for (const child of lineNode.children) {
    if (child.type !== "element") {
      newChildren.push(child);
      continue;
    }

    const spanChildren = [...child.children];

    let leadingSpaces = "";
    const firstChild = spanChildren[0];
    if (firstChild?.type === "text") {
      const [spaces, rest] = takeLeadingSpaces(firstChild.value);
      if (spaces) {
        leadingSpaces = spaces;
        if (rest) spanChildren[0] = { type: "text", value: rest };
        else spanChildren.shift();
      }
    }

    let trailingSpaces = "";
    const lastChild = spanChildren[spanChildren.length - 1];
    if (lastChild?.type === "text") {
      const [rest, spaces] = takeTrailingSpaces(lastChild.value);
      if (spaces) {
        trailingSpaces = spaces;
        if (rest)
          spanChildren[spanChildren.length - 1] = { type: "text", value: rest };
        else spanChildren.pop();
      }
    }

    const contentLength = spanChildren.reduce(
      (sum, c) => sum + getTextLength(c),
      0,
    );

    const makeToken = (
      props: Record<string, unknown>,
      c: ElementContent[],
    ): Element => ({
      ...child,
      properties: {
        ...child.properties,
        ...(child.properties.class
          ? { class: [...(child.properties.class as string[]), "diff-token"] }
          : { class: ["diff-token"] }),
        ...props,
      },
      children: c,
    });

    if (leadingSpaces) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + leadingSpaces.length,
          },
          [{ type: "text", value: leadingSpaces }],
        ),
      );
      charOffset += leadingSpaces.length;
    }
    if (spanChildren.length > 0) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + contentLength,
          },
          spanChildren,
        ),
      );
      charOffset += contentLength;
    }
    if (trailingSpaces) {
      newChildren.push(
        makeToken(
          {
            "data-char-start": charOffset,
            "data-char-end": charOffset + trailingSpaces.length,
          },
          [{ type: "text", value: trailingSpaces }],
        ),
      );
      charOffset += trailingSpaces.length;
    }
  }

  lineNode.children = newChildren;
}

function takeLeadingSpaces(value: string): [spaces: string, rest: string] {
  let i = 0;
  while (i < value.length && (value[i] === " " || value[i] === "\t")) i++;
  return [value.slice(0, i), value.slice(i)];
}

function takeTrailingSpaces(value: string): [rest: string, spaces: string] {
  let i = value.length;
  while (i > 0 && (value[i - 1] === " " || value[i - 1] === "\t")) i--;
  return [value.slice(0, i), value.slice(i)];
}
