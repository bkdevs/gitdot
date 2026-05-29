"use client";

import type {
  RepositoryBlobResource,
  RepositoryCommitResource,
} from "gitdot-api";
import {
  ClientProvider,
  type DiffEntry,
  fetchFileBlobs,
} from "gitdot-dal/client";
import { useEffect, useState } from "react";
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
  const [blobs, setBlobs] = useState<Record<string, RepositoryBlobResource>>(
    {},
  );
  const [entry, setEntry] = useState<DiffEntry | null>(null);
  const { hoveredSha, selectedSha } = useFileViewerContext();

  useEffect(() => {
    if (fileCommits.length === 0) return;

    const shas = fileCommits.map((c) => c.sha);
    fetchFileBlobs(owner, repo, path, shas).then((result) => {
      setBlobs(Object.fromEntries(result.map((b) => [b.commit_sha, b])));
    });
  }, [fileCommits, owner, repo, path]);

  const activeSha = selectedSha ?? hoveredSha;
  const activeCommit = activeSha
    ? fileCommits.find((c) => c.sha === activeSha)
    : null;

  useEffect(() => {
    if (!activeSha || !blobs[activeSha]) {
      setEntry(null);
      return;
    }
    const next = blobs[activeSha];
    const index = fileCommits.findIndex((c) => c.sha === activeSha);
    const old =
      index >= 0 && index < fileCommits.length - 1
        ? (blobs[fileCommits[index + 1].sha] ?? null)
        : null;

    let cancelled = false;
    setEntry(null);
    ClientProvider.instance.renderBlob(old, next).then((result) => {
      if (!cancelled) setEntry(result);
    });
    return () => {
      cancelled = true;
    };
  }, [activeSha, blobs, fileCommits]);

  return (
    <div className="flex w-full h-full min-h-0 overflow-hidden">
      <div
        data-page-scroll
        className="flex-1 min-w-0 overflow-auto scrollbar-thin"
      >
        {activeCommit && entry ? (
          <FileCommitBody commit={activeCommit} entry={entry} />
        ) : (
          <FileBody />
        )}
      </div>
      <FileCommits commits={fileCommits} path={path} />
    </div>
  );
}
