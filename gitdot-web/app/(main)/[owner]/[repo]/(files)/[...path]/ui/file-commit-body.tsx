"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import type { DiffEntry } from "@/actions";
import { DiffBody } from "../../../commits/[sha]/ui/diff-body";
import { FileCommitHeader } from "./file-commit-header";

export function FileCommitBody({
  commit,
  entry,
}: {
  commit: RepositoryCommitResource;
  entry: DiffEntry;
}) {
  return (
    <div className="flex flex-col w-full">
      <FileCommitHeader commit={commit} />
      <DiffBody spans={entry.spans} />
    </div>
  );
}
