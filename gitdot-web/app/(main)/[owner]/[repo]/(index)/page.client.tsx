"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  resolveResources,
} from "@/provider/client";
import type { Resources } from "./page";
import { MarkdownBody } from "../ui/markdown/markdown-body";

type ResourceRequests = ResourceRequestsType<Resources>;
type ResourcePromises = ResourcePromisesType<Resources>;

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
  return (
    <Suspense>
      <PageContent promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({ promises }: { promises: ResourcePromises }) {
  const readme = use(promises.readme);

  if (!readme || readme.type !== "file") {
    return <div className="p-2 text-sm">README.md not found</div>;
  }
  return (
    <div className="p-4 max-w-4xl">
      <MarkdownBody content={readme.content} />
    </div>
  );
}
