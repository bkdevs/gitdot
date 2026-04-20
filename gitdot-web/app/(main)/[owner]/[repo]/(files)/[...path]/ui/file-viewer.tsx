"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { useEffect, useState } from "react";
import type { DiffEntry } from "@/actions";
import { renderBlobDiffsAction } from "@/actions/diff";
import { FileBody } from "./file-body";
import { FileCommitBody } from "./file-commit-body";
import { FileCommits } from "./file-commits";
import { useFileViewerContext } from "./file-viewer-context";

export function FileViewer({
  fileCommits,
  owner,
  repo,
  path,
}: {
  fileCommits: RepositoryCommitResource[];
  owner: string;
  repo: string;
  path: string;
}) {
  const [diffEntries, setDiffEntries] = useState<Record<string, DiffEntry>>({});
  const { hoveredSha, selectedSha } = useFileViewerContext();

  useEffect(() => {
    const shas = fileCommits.map((c) => c.sha);
    if (shas.length === 0) return;
    renderBlobDiffsAction(owner, repo, shas, path).then(setDiffEntries);
  }, [fileCommits, owner, repo, path]);

  const activeSha = selectedSha ?? hoveredSha;
  const activeCommit = activeSha
    ? fileCommits.find((c) => c.sha === activeSha)
    : null;
  const activeDiffEntry = activeSha ? (diffEntries[activeSha] ?? null) : null;

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div
        data-page-scroll
        className="flex-1 min-w-0 overflow-auto scrollbar-thin"
      >
        {activeCommit && activeDiffEntry ? (
          <FileCommitBody commit={activeCommit} entry={activeDiffEntry} />
        ) : (
          <FileBody />
        )}
      </div>
      <FileCommits commits={fileCommits} path={path} />
    </div>
  );
}
