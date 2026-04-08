"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import type { DiffEntry } from "@/actions";
import { DiffBody } from "../../../commits/[sha]/ui/diff-body";
import { FileCommitHeader } from "./file-commit-header";

export function FileCommitBody({
  commit,
  diffEntry,
}: {
  commit: RepositoryCommitResource;
  diffEntry: DiffEntry;
}) {
  return (
    <div className="flex flex-col w-full">
      <FileCommitHeader commit={commit} />
      <DiffBody data={diffEntry.data} />
    </div>
  );
}
