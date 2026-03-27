"use client";

import { RepositoryCommitResource } from "gitdot-api";
import { Root } from "hast";
import type { LineSelection } from "../util";
import { FileBody } from "./file-body";
import { FileCommits } from "./file-commits";

export function FileViewer({
  hast,
  fileCommits,
  selectedLines,
}: {
  hast: Root;
  fileCommits: RepositoryCommitResource[];
  selectedLines: LineSelection | null;
}) {
  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div
        data-page-scroll
        className="flex-1 min-w-0 overflow-auto scrollbar-thin"
      >
        <FileBody selectedLines={selectedLines} hast={hast} />
      </div>
      <FileCommits commits={fileCommits} />
    </div>
  );
}
