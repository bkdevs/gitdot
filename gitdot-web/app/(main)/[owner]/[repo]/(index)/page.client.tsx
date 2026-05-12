"use client";

import { Suspense, use } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import { MarkdownBody } from "../ui/markdown/markdown-body";
import type { Resources } from "./page";
import { RepoPanel } from "./ui/repo-panel";

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
  const resolvedPromises = useResolvePromises(owner, repo, requests, promises);
  return (
    <Suspense fallback={<Loading />}>
      <PageContent promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({ promises }: { promises: ResourcePromises }) {
  const readme = use(promises.readme);

  return (
    <div className="flex h-full w-full overflow-hidden">
      <div className="p-4 w-full max-w-4xl shrink-0 overflow-y-auto scrollbar-none">
        {readme && readme.type === "file" ? (
          <MarkdownBody content={readme.content} />
        ) : (
          <div className="p-2 text-sm">README.md not found</div>
        )}
      </div>
      <RepoPanel />
    </div>
  );
}
