"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { Suspense, use, useState } from "react";
import {
  type ResourcePromisesType,
  type ResourceRequestsType,
  useResolvePromises,
} from "@/(main)/[owner]/[repo]/resources";
import { Loading } from "@/ui/loading";
import type { Resources } from "./page";
import { BuildRow } from "./ui/build-row";
import { BuildsHeader } from "./ui/builds-header";

export type BuildsFilter = "main" | "pull-request";

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
      <PageContent owner={owner} repo={repo} promises={resolvedPromises} />
    </Suspense>
  );
}

function PageContent({
  owner,
  repo,
  promises,
}: {
  owner: string;
  repo: string;
  promises: ResourcePromises;
}) {
  const builds = use(promises.builds);
  const commits = use(promises.commits);
  if (!builds || !commits) return null;

  const [filter, setFilter] = useState<BuildsFilter>("main");
  const commitsBySha: Record<string, RepositoryCommitResource> = {};
  for (const commit of commits) {
    commitsBySha[commit.sha] = commit;
  }
  const filteredBuilds = builds.filter((build) => {
    if (!commitsBySha[build.commit_sha]) return false;
    if (filter === "main") return build.trigger === "push_to_main";
    return build.trigger === "pull_request";
  });

  return (
    <div className="flex flex-col">
      <BuildsHeader
        owner={owner}
        repo={repo}
        filter={filter}
        setFilter={setFilter}
      />
      {filteredBuilds.map((build) => (
        <BuildRow
          key={build.id}
          owner={owner}
          repo={repo}
          build={build}
          commit={commitsBySha[build.commit_sha]}
        />
      ))}
    </div>
  );
}
