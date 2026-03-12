"use client";

import { Suspense } from "react";
import { RepoFileDialogWrapper } from "./repo-file-dialog";

export function RepoDialogs({ owner, repo }: { owner: string; repo: string }) {
  return (
    <Suspense fallback={null}>
      <RepoFileDialogWrapper owner={owner} repo={repo} />
    </Suspense>
  );
}
