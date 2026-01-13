"use client";

import { Suspense, useEffect, useState } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { RepoFileDialog } from "./repo-file-dialog";

export function RepoDialogs({
  repo,
  files,
  filePreviewsPromise,
}: {
  repo: string;
  files: RepositoryTreeEntry[];
  filePreviewsPromise: Promise<Map<string, string>>;
}) {
  const [fileDialogOpen, setFileDialogOpen] = useState(false);
  const [previewsReady, setPreviewsReady] = useState(false);

  useEffect(() => {
    filePreviewsPromise.then(() => setPreviewsReady(true));
  }, [filePreviewsPromise]);

  useEffect(() => {
    if (!previewsReady) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "p") {
        const target = e.target as HTMLElement;
        if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
          return;
        }
        e.preventDefault();
        setFileDialogOpen(true);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [previewsReady]);

  return (
    <Suspense fallback={null}>
      <RepoFileDialog
        open={fileDialogOpen}
        setOpen={setFileDialogOpen}
        repo={repo}
        files={files}
        filePreviewsPromise={filePreviewsPromise}
      />
    </Suspense>
  );
}
