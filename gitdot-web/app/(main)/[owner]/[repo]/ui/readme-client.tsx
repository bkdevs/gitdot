"use client";

import { Suspense, use } from "react";
import { useRepoContext } from "../context";
import { MarkdownBody } from "./markdown/markdown-body";

function ReadmeContent() {
  const { blobs } = useRepoContext();
  const resolved = use(blobs);
  const readme = resolved?.blobs.find((b) => b.path === "README.md");

  if (!readme || readme.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}

export function ReadmeClient() {
  return (
    <Suspense>
      <ReadmeContent />
    </Suspense>
  );
}
