"use client";

import type { CommitAuthorResource } from "gitdot-api";
import { type DiffEntry, fetchCommitDiff } from "gitdot-dal/client";
import { useEffect, useState } from "react";
import { CommitBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/commit-body";
import { CommitHeader } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/commit-header";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { Loading } from "@/ui/loading";

export type OpenCommitDialogDetail = {
  commit: {
    owner_name: string;
    repo_name: string;
    sha: string;
    message: string;
    date: string;
    author: CommitAuthorResource;
  };
};

const diffCache = new Map<string, DiffEntry[]>();

/** Warm the diff cache so the dialog opens instantly. Fire-and-forget. */
export function prefetchCommitDiff(owner: string, repo: string, sha: string) {
  const key = `${owner}/${repo}/${sha}`;
  if (diffCache.has(key)) return;
  fetchCommitDiff(owner, repo, sha)
    .then((entries) => diffCache.set(key, entries))
    .catch(() => {});
}

export function CommitDialog() {
  const [open, setOpen] = useState(false);
  const [commit, setCommit] = useState<OpenCommitDialogDetail["commit"] | null>(
    null,
  );
  const [diffEntries, setDiffEntries] = useState<DiffEntry[] | null>(null);

  useEffect(() => {
    const handler = (e: Event) => {
      const detail = (e as CustomEvent<OpenCommitDialogDetail>).detail;
      const { owner_name, repo_name, sha } = detail.commit;
      const key = `${owner_name}/${repo_name}/${sha}`;
      setCommit(detail.commit);
      setOpen(true);

      const cached = diffCache.get(key);
      if (cached) {
        setDiffEntries(cached);
        return;
      }
      setDiffEntries(null);
      fetchCommitDiff(owner_name, repo_name, sha).then((result) => {
        diffCache.set(key, result);
        setDiffEntries(result);
      });
    };
    window.addEventListener("openCommitDialog", handler);
    return () => window.removeEventListener("openCommitDialog", handler);
  }, []);

  if (!commit) return null;
  const shortSha = commit.sha.substring(0, 7);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        showOverlay={false}
        className="sm:max-w-none max-w-[80vw] w-[80vw] h-[90vh] max-h-[90vh] top-[48%] p-0 gap-0 flex flex-col border-black rounded-xs shadow-2xl overflow-hidden"
      >
        <DialogTitle className="sr-only">Commit {shortSha}</DialogTitle>

        <div className="flex-1 overflow-y-auto scrollbar-thin">
          <div
            data-diff-top
            className="w-full px-6 pt-4 pb-8 flex flex-col gap-6"
          >
            <CommitHeader
              owner={commit.owner_name}
              repo={commit.repo_name}
              sha={commit.sha}
              message={commit.message}
              date={commit.date}
              author={commit.author}
              showOpenInTab
            />
            {diffEntries ? <CommitBody entries={diffEntries} /> : <Loading />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
