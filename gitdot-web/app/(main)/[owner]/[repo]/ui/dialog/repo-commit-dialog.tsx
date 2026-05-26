"use client";

import type { RepositoryCommitResource } from "gitdot-api";
import { Suspense, use, useEffect, useState } from "react";
import { CommitBody } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/commit-body";
import { CommitHeader } from "@/(main)/[owner]/[repo]/commits/[sha]/ui/commit-header";
import { type DiffEntry, renderCommitDiffAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { Loading } from "@/ui/loading";

export type OpenCommitDialogDetail = {
  commit: RepositoryCommitResource;
};

export function RepoCommitDialog({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const [open, setOpen] = useState(false);
  const [commit, setCommit] = useState<RepositoryCommitResource | null>(null);
  const [diffPromise, setDiffPromise] = useState<Promise<DiffEntry[]> | null>(
    null,
  );

  useEffect(() => {
    const handler = (e: Event) => {
      const detail = (e as CustomEvent<OpenCommitDialogDetail>).detail;
      setCommit(detail.commit);
      setDiffPromise(renderCommitDiffAction(owner, repo, detail.commit.sha));
      setOpen(true);
    };
    window.addEventListener("openCommitDialog", handler);
    return () => window.removeEventListener("openCommitDialog", handler);
  }, [owner, repo]);

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
              commit={commit}
              owner={owner}
              repo={repo}
              showOpenInTab
            />
            {diffPromise && (
              <Suspense fallback={<Loading />}>
                <CommitBodyAsync promise={diffPromise} />
              </Suspense>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}

function CommitBodyAsync({ promise }: { promise: Promise<DiffEntry[]> }) {
  const entries = use(promise);
  return <CommitBody entries={entries} />;
}
