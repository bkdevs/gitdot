import type { RepositoryDiffStatResource } from "gitdot-api";
import { use } from "react";
import type { DiffData } from "@/actions/repository";
import { DiffBody } from "./diff-body";

export function CommitBody({
  diffs,
  diffData,
}: {
  diffs: RepositoryDiffStatResource[];
  diffData: Promise<Record<string, DiffData>>;
}) {
  const diffMap = use(diffData);

  return (
    <div className="flex flex-col">
      {diffs.map((stat) => (
        <DiffBody key={stat.path} stat={stat} diff={diffMap[stat.path]} />
      ))}
    </div>
  );
}
