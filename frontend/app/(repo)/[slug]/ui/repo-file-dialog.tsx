"use client";

import { useEffect, useState } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function RepoFileDialog({
  folders,
  entries,
}: {
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
}) {
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (
        e.key === "p" &&
        !e.metaKey &&
        !e.ctrlKey &&
        !e.altKey &&
        !e.shiftKey
      ) {
        const target = e.target as HTMLElement;
        if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
          return;
        }
        e.preventDefault();
        setOpen(true);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-[80vw]! max-h-[85vh]! w-full h-full"
        showOverlay={false}
      >
        <DialogTitle>Files</DialogTitle>
      </DialogContent>
    </Dialog>
  );
}
