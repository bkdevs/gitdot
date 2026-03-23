"use client";

import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import { useState } from "react";
import { BuildRow } from "./build-row";
import { BuildsHeader } from "./builds-header";

export type BuildsFilter = "main" | "pull-request";

export function BuildsClient({
  owner,
  repo,
  builds,
  commits,
}: {
  owner: string;
  repo: string;
  builds: BuildResource[];
  commits: RepositoryCommitResource[];
}) {
  const [filter, setFilter] = useState<BuildsFilter>("main");

  const commitsBySha: Record<string, RepositoryCommitResource> = {};
  for (let i = 0; i < builds.length; i++) {
    commitsBySha[builds[i].commit_sha] = commits[i];
  }
  const filteredBuilds = builds.filter((build) => {
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
