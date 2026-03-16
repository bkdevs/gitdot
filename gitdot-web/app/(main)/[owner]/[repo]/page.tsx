"use client";

import { Suspense, use } from "react";
import { useRepoContext } from "./context";
import { MarkdownBody } from "./ui/markdown/markdown-body";

function ReadmeContent() {
  const { readme } = useRepoContext();
  const resolved = use(readme);

  if (!resolved || resolved.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={resolved.content} />
    </div>
  );
}

export default function Page() {
  return (
    <Suspense>
      <ReadmeContent />
    </Suspense>
  );
}
