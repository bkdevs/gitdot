"use client";

import { use } from "react";
import { resolveResources } from "@/provider/client";
import type { ResourcePromises, ResourceRequests } from "./page";
import { MarkdownBody } from "./ui/markdown/markdown-body";

export function PageClient({
  owner,
  repo,
  requests,
  promises,
}: {
  owner: string;
  repo: string;
  requests: ResourceRequests;
  promises: ResourcePromises;
}) {
  const resolvedPromises = resolveResources(owner, repo, requests, promises);
  const readme = use(resolvedPromises.readme);

  if (!readme || readme.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}
