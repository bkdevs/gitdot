import type { RepositoryDiffStatResource } from "gitdot-api";
import { Suspense } from "react";
import type { DiffData } from "@/actions/repository";
import { DiffBody } from "./diff-body";
import { DiffFileClient } from "./diff-file-client";

export function CommitBody({
  diffs,
  allDiffDataPromise,
}: {
  diffs: RepositoryDiffStatResource[];
  allDiffDataPromise: Promise<Record<string, DiffData>>;
}) {
  return (
    <div className="flex flex-col">
      {diffs.map((stat) => (
        <DiffFileClient
          key={stat.path}
          leftPath={stat.path}
          rightPath={stat.path}
          linesAdded={stat.lines_added}
          linesRemoved={stat.lines_removed}
        >
          <DiffBody path={stat.path} allDiffDataPromise={allDiffDataPromise} />
        </DiffFileClient>
      ))}
    </div>
  );
}
