import { inferLanguage, pairLines } from "@/(repo)/[slug]/util";
import type { RepositoryFileDiff } from "@/lib/dto";
import { Element, Root } from "hast";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import type { JSX } from "react";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
import { codeToHast } from "shiki";
import { DiffLine } from "./diff-line";

async function renderContent(
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
          node.type = "element"
        },
      },
    ],
  });

  const root = hast as Root;
  const pre = root.children[0] as Element;
  const code = pre.children[0] as Element;

  return code.children.filter((child): child is Element => child.type === 'element');
}

export async function FileDiff({ diff }: { diff: RepositoryFileDiff }) {
  const { left, right, chunks } = diff;
  const path = left?.path || right?.path;
  if (!path) {
    throw new Error("File path or chunks are missing");
  }
  if (!left?.content || !right?.content) {
    return <div>
      one of the two files are missing, this is unimplemented as of now, should show a single file view
    </div>
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
    renderContent(language, left.content),
    renderContent(language, right.content)
  ]);

  console.log(JSON.stringify(leftSpans.slice(0, 1)));

  const emptySpan: Element = {
    type: "element",
    tagName: "diffline",
    properties: {
      class: "line w-full",
      "data-line-type": "sentinel"
    },
    children: []
  };

  const chunkComponents = [];

  for (const chunk of chunks) {
    const leftSpansChunk: Element[] = [];
    const rightSpansChunk: Element[] = [];
    const pairedLines = pairLines(chunk);

    for (const [left, right] of pairedLines) {
      leftSpansChunk.push(left ? leftSpans[left] : emptySpan);
      rightSpansChunk.push(right ? rightSpans[right] : emptySpan);
    }

    const container: Element = {
      type: "element",
      tagName: "div",
      properties: {
        className: "flex w-full mb-8 border-t border-b border-border"
      },
      children: [
        {
          type: "element",
          tagName: "pre",
          properties: {
            className: "flex flex-col w-1/2 overflow-auto border-border border-r text-sm font-mono"
          },
          children: leftSpansChunk
        },
        {
          type: "element",
          tagName: "pre",
          properties: {
            className: "flex flex-col overflow-auto w-1/2 text-sm font-mono"
          },
          children: rightSpansChunk
        }
      ]
    };

    const chunkElement = toJsxRuntime(container, {
      Fragment,
      jsx,
      jsxs,
      components: {
        diffline: (props) => <DiffLine {...props} />,
      },
    }) as JSX.Element;

    chunkComponents.push(chunkElement);
  }

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full h-9 items-center px-2 border-b border-t border-border text-sm font-mono sticky top-0 z-10 bg-sidebar">
        {path}
      </div>
      {chunkComponents.map((chunkElement) => chunkElement)}
    </div>
  );
}
