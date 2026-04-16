import { renderMermaidSVG } from "beautiful-mermaid";
import type { Element, ElementContent } from "hast";
import React from "react";
import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Link from "@/ui/link";
import { highlightMarkdownCode } from "./markdown-highlighter";

function hastText(node: ElementContent): string {
  if (node.type === "text") return node.value;
  if (node.type === "element") return node.children.map(hastText).join("");
  return "";
}

function extractText(children: React.ReactNode): string {
  return React.Children.toArray(children)
    .map((c) =>
      typeof c === "string"
        ? c
        : extractText(
            (c as React.ReactElement<{ children?: React.ReactNode }>).props
              ?.children ?? "",
          ),
    )
    .join("");
}

function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .replace(/[\s_]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

export function MarkdownBody({
  content,
  compact = false,
}: {
  content: string;
  compact?: boolean;
}) {
  return (
    <Markdown
      remarkPlugins={[remarkGfm]}
      components={{
        h1: ({ node, children, ...props }) => (
          <h1
            id={slugify(extractText(children))}
            className={
              compact
                ? "text-lg font-bold mb-1 border-b pb-1"
                : "text-3xl font-bold mb-4 border-b pb-2"
            }
            {...props}
          >
            {children}
          </h1>
        ),
        h2: ({ node, children, ...props }) => (
          <h2
            id={slugify(extractText(children))}
            className={
              compact
                ? "text-base font-semibold mb-1"
                : "text-xl font-semibold mb-3"
            }
            {...props}
          >
            {children}
          </h2>
        ),
        h3: ({ node, children, ...props }) => (
          <h3
            id={slugify(extractText(children))}
            className={
              compact ? "text-sm font-medium mb-1" : "text-lg font-medium mb-2"
            }
            {...props}
          >
            {children}
          </h3>
        ),
        h4: ({ node, children, ...props }) => (
          <h4
            id={slugify(extractText(children))}
            className={
              compact
                ? "text-sm font-medium mb-0.5"
                : "text-base font-medium mb-2"
            }
            {...props}
          >
            {children}
          </h4>
        ),
        h5: ({ node, children, ...props }) => (
          <h5
            id={slugify(extractText(children))}
            className={
              compact
                ? "text-xs font-semibold mb-0.5"
                : "text-sm font-semibold mb-2"
            }
            {...props}
          >
            {children}
          </h5>
        ),

        p: ({ node, ...props }) => (
          <p
            className={
              compact ? "text-sm mb-1.5" : "leading-relaxed text-sm mb-4"
            }
            {...props}
          />
        ),
        a: ({ node, href, children, ...props }) => (
          <Link
            href={href ?? ""}
            className="text-sm underline underline-offset-4 decoration-1 hover:decoration-2 transition-all"
            {...props}
          >
            {children}
          </Link>
        ),
        blockquote: ({ node, ...props }) => (
          <blockquote
            className={
              compact
                ? "text-sm border-l-2 border-current pl-2 italic my-1.5 opacity-80"
                : "text-sm border-l-4 border-current pl-4 italic my-4 opacity-80"
            }
            {...props}
          />
        ),

        ul: ({ node, ...props }) => (
          <ul
            className={
              compact
                ? "list-disc list-outside ml-4 mb-1.5 space-y-0"
                : "list-disc list-outside ml-6 mb-4 space-y-1"
            }
            {...props}
          />
        ),
        ol: ({ node, ...props }) => (
          <ol
            className={
              compact
                ? "list-decimal list-outside ml-4 mb-1.5 space-y-0"
                : "list-decimal list-outside ml-6 mb-4 space-y-1"
            }
            {...props}
          />
        ),
        li: ({ node, ...props }) => <li className="text-sm" {...props} />,

        pre: ({ node, children, ...props }) => {
          const codeEl = node?.children?.find(
            (child): child is Element =>
              child.type === "element" && child.tagName === "code",
          );
          const classNames = Array.isArray(codeEl?.properties?.className)
            ? (codeEl.properties.className as string[])
            : [];
          const lang =
            classNames
              .find((c) => c.startsWith("language-"))
              ?.replace("language-", "") ?? "";

          if (lang === "mermaid") return <>{children}</>;

          if (codeEl && lang) {
            const code = hastText(codeEl).trimEnd();
            const html = highlightMarkdownCode(code, lang);
            if (html) {
              return (
                // biome-ignore lint/security/noDangerouslySetInnerHtml: server-generated Shiki HTML
                <div dangerouslySetInnerHTML={{ __html: html }} />
              );
            }
          }

          return (
            <pre
              className={
                compact
                  ? "bg-black/5 dark:bg-white/10 rounded p-2 mb-1.5 overflow-x-auto text-xs"
                  : "bg-black/5 dark:bg-white/10 rounded p-4 mb-4 overflow-x-auto text-sm"
              }
              style={{
                fontFamily:
                  "ui-monospace, 'Cascadia Code', 'Fira Code', Menlo, Consolas, monospace",
              }}
              {...props}
            >
              {children}
            </pre>
          );
        },
        code: ({ node, className, children, ...props }) => {
          if (className === "language-mermaid") {
            try {
              const svg = renderMermaidSVG(String(children).trimEnd(), {
                transparent: true,
                fg: "#09090B",
                muted: "#52525B",
              });
              return (
                <div
                  className={
                    compact
                      ? "my-1.5 flex justify-center overflow-x-auto"
                      : "my-4 flex justify-center overflow-x-auto"
                  }
                  // biome-ignore lint/security/noDangerouslySetInnerHtml: beautiful-mermaid renders trusted SVG server-side
                  dangerouslySetInnerHTML={{ __html: svg }}
                />
              );
            } catch {
              // fall through to plain code block on parse error
            }
          }
          const isBlock =
            node?.position?.start.line !== node?.position?.end.line;
          return (
            <code
              className={
                isBlock
                  ? compact
                    ? "text-xs"
                    : "text-sm"
                  : "bg-black/5 dark:bg-white/10 px-1.5 py-0.5 rounded font-mono text-sm"
              }
              style={
                isBlock
                  ? {
                      fontFamily:
                        "ui-monospace, 'Cascadia Code', 'Fira Code', Menlo, Consolas, monospace",
                      fontSize: compact ? "0.7rem" : "0.8125rem",
                    }
                  : undefined
              }
              {...props}
            >
              {children}
            </code>
          );
        },

        table: ({ node, ...props }) => (
          <div
            className={
              compact
                ? "text-xs overflow-x-auto mb-2"
                : "text-sm overflow-x-auto mb-6"
            }
          >
            <table
              className="min-w-full divide-y divide-current border border-current/20"
              {...props}
            />
          </div>
        ),
        th: ({ node, ...props }) => (
          <th
            className={
              compact
                ? "px-2 py-1.5 text-left text-xs font-semibold bg-black/5 dark:bg-white/5"
                : "px-3 py-3.5 text-left text-sm font-semibold bg-black/5 dark:bg-white/5"
            }
            {...props}
          />
        ),
        td: ({ node, ...props }) => (
          <td
            className={
              compact
                ? "px-2 py-1.5 text-xs border-t border-current/10"
                : "px-3 py-4 text-sm border-t border-current/10"
            }
            {...props}
          />
        ),

        img: ({ node, ...props }) => (
          // biome-ignore lint/performance/noImgElement: react-markdown img renderer needs native img; next/image requires known dimensions
          <img
            className={
              compact
                ? "rounded my-2 mx-auto max-w-full h-auto"
                : "rounded-xl my-8 mx-auto max-w-full h-auto"
            }
            {...props}
            alt={props.alt || ""}
          />
        ),
        hr: () => (
          <hr
            className={
              compact
                ? "my-2 border-t border-current/20"
                : "my-8 border-t border-current/20"
            }
          />
        ),
      }}
    >
      {content}
    </Markdown>
  );
}
