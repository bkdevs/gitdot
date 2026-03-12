"use client";

import { Suspense, useEffect, useState } from "react";
import { useRepoResource } from "../../context";
import { parseRepositoryTree } from "../../util";
import { RepoFileDialog } from "./repo-file-dialog";

export function RepoDialogs({
  owner,
  repo,
  previewsPromise,
}: {
  owner: string;
  repo: string;
  previewsPromise: Promise<Map<string, string>>;
}) {
  const [fileDialogOpen, setFileDialogOpen] = useState(false);
  const [previewsReady, setPreviewsReady] = useState(false);

  const tree = useRepoResource("tree");
  const { entries } = parseRepositoryTree(tree);
  const files = Array.from(entries.values()).filter(
    (entry) => entry.entry_type === "blob",
  );

  useEffect(() => {
    previewsPromise.then(() => setPreviewsReady(true));
  }, [previewsPromise]);

  useEffect(() => {
    if (!previewsReady) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "p" || e.key === "/") {
        const target = e.target as HTMLElement;
        if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
          return;
        }
        e.preventDefault();
        setFileDialogOpen(true);
      }
    };

    const handleOpenFileSearch = () => setFileDialogOpen(true);

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("openFileSearch", handleOpenFileSearch);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("openFileSearch", handleOpenFileSearch);
    };
  }, [previewsReady]);

  return (
    <Suspense fallback={null}>
      <RepoFileDialog
        open={fileDialogOpen}
        setOpen={setFileDialogOpen}
        owner={owner}
        repo={repo}
        files={files}
        previewsPromise={previewsPromise}
      />
    </Suspense>
  );
}
