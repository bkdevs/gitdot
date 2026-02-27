"use client";

import type { BuildResource } from "gitdot-api-ts";
import { BuildRow } from "./build-row";
import { BuildsHeader } from "./builds-header";

export function BuildsClient({
  owner,
  repo,
  builds,
}: {
  owner: string;
  repo: string;
  builds: BuildResource[];
}) {
  return (
    <div className="flex flex-col">
      <BuildsHeader owner={owner} repo={repo} />
      {builds.map((build) => (
        <BuildRow key={build.id} owner={owner} repo={repo} build={build} />
      ))}
    </div>
  );
}
