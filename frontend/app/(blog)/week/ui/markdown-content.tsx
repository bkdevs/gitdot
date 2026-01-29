"use client";

import ReactMarkdown from "react-markdown";
import rehypeRaw from "rehype-raw";
import { ImageContent } from "./image-content";
import { VideoContent } from "./video-content";

export default function MarkdownContent({ content }: { content: string }) {
  return (
    <ReactMarkdown
      rehypePlugins={[rehypeRaw]}
      components={{
        h1: ({ children }) => <h1 className="text-2xl mb-4">{children}</h1>,
        h2: ({ children }) => <h2 className="text-xl mb-3">{children}</h2>,
        h3: ({ children }) => <h3 className="text-lg mb-2">{children}</h3>,
        p: ({ children }) => <p className="mb-4">{children}</p>,
        ul: ({ children }) => <ul className="mb-2">{children}</ul>,
        ol: ({ children }) => (
          <ol className="mb-2 list-decimal ml-6">{children}</ol>
        ),
        li: ({ children }) => <li className="ml-4">{children}</li>,
        a: ({ children, href }) => (
          <a
            href={href}
            className="underline"
            target="_blank"
            rel="noopener noreferrer"
          >
            {children}
          </a>
        ),
        code: ({ children, className, node }) => {
          // Check if this is a code block (inside pre) vs inline code
          // Code blocks have className with "language-" OR their parent is a <pre> element
          const isCodeBlock =
            className?.includes("language-") ||
            node?.position?.start.line !== node?.position?.end.line ||
            (typeof children === "string" && children.includes("\n"));
          if (isCodeBlock) {
            return <code className="font-mono text-sm">{children}</code>;
          }
          return (
            <code className="bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded font-mono text-sm">
              {children}
            </code>
          );
        },
        pre: ({ children }) => (
          <pre className="bg-gray-100 dark:bg-gray-800 p-4 rounded overflow-x-auto mb-4">
            {children}
          </pre>
        ),
        strong: ({ children }) => (
          <strong className="font-bold">{children}</strong>
        ),
        em: ({ children }) => <em className="italic">{children}</em>,
        img: ImageContent,
        video: VideoContent,
      }}
    >
      {content}
    </ReactMarkdown>
  );
}
