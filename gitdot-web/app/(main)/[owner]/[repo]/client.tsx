"use client";

import { useEffect, useState } from "react";
import type { RepositoryBlobResource } from "gitdot-api";
import { usePageContext } from "./shell";
import { MarkdownBody } from "./ui/markdown/markdown-body";

export function Client() {
  const context = usePageContext();
  const [readme, setReadme] = useState<RepositoryBlobResource | null>(null);

  useEffect(() => {
    context.readme.then(setReadme);
  }, [context.readme]);

  if (readme === null) return null;
  if (readme.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}
