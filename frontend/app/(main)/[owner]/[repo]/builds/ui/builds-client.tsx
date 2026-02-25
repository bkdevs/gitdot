"use client";

import type { BuildsResponse } from "@/lib/dto";
import { BuildRow } from "./build-row";
import { BuildsHeader } from "./builds-header";

export function BuildsClient({
  owner,
  repo,
  builds,
}: {
  owner: string;
  repo: string;
  builds: BuildsResponse;
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
