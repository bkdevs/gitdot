"use client";

import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import { useTimezone } from "@/(main)/context/timezone";
import { formatDateTime } from "@/util/date";
import { JobTimer } from "./job-timer";

export function BuildDetails({
  build,
  commit,
}: {
  build: BuildResource;
  commit: RepositoryCommitResource | null;
}) {
  const tz = useTimezone();
  const createdAt = new Date(build.created_at);
  const updatedAt = new Date(build.updated_at);
  const running = build.status === "running";

  return (
    <div className="flex h-full w-1/4 flex-col border-l">
      <div className="space-y-2 p-2">
        <div>
          <div className="text-xs text-muted-foreground">Commit</div>
          <div className="truncate text-sm">
            {commit?.message ?? build.commit_sha.slice(0, 7)}
          </div>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Author</div>
          <div className="text-sm">
            {commit?.author.name ?? commit?.author.git_name}
          </div>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Created</div>
          <div className="text-sm">{formatDateTime(createdAt, tz)}</div>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Runtime</div>
          <div className="text-sm">
            <JobTimer
              createdAt={createdAt}
              updatedAt={updatedAt}
              running={running}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
