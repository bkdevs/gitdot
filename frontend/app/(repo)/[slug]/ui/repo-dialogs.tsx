"use client";

import { useEffect, useState } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { RepoFileDialog } from "./repo-file-dialog";

export function RepoDialogs({
  repo,
  folders,
  entries,
}: {
  repo: string;
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
}) {
  const [fileDialogOpen, setFileDialogOpen] = useState(false);
  const files = Array.from(entries.values()).filter(
    (entry) => entry.entry_type === "blob",
  );

  useEffect(() => {
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
  }, []);

  return (
    <RepoFileDialog
      open={fileDialogOpen}
      setOpen={setFileDialogOpen}
      repo={repo}
      files={files}
    />
  );
}
