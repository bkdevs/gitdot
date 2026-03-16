"use client";

import { use } from "react";
import { usePageContext } from "./shell";
import { MarkdownBody } from "./ui/markdown/markdown-body";

export function Client() {
  const context = usePageContext();
  const readme = use(context.readme);

  if (!readme || readme.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}
