import type { BuildResource, RepositoryCommitResource } from "gitdot-api";
import { FileCog } from "lucide-react";
import { formatDateTime } from "@/util";
import { JobTimer } from "./job-timer";

export function BuildDetails({
  build,
  commit,
}: {
  build: BuildResource;
  commit: RepositoryCommitResource | null;
}) {
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
          <div className="text-sm">{commit?.author.name}</div>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Created</div>
          <div className="text-sm">{formatDateTime(createdAt)}</div>
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
      <div className="mt-auto flex w-full items-center justify-end border-t border-border px-2">
        <button
          type="button"
          className="flex h-8 items-center justify-center gap-1.5 rounded-none border-l border-border bg-background pl-2 pr-1 text-xs text-foreground outline-0! ring-0! hover:bg-accent/50"
        >
          <FileCog className="size-3" />
          Config
        </button>
      </div>
    </div>
  );
}
