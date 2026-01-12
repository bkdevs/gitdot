"use client";

import { useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { fuzzyMatch, getMockPreview } from "../util";

export function RepoFileDialog({
  open,
  setOpen,
  repo,
  files,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  repo: string;
  files: RepositoryTreeEntry[];
}) {
  const router = useRouter();
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const filteredFiles = useMemo(() => {
    if (!query) return files;
    return files.filter((file) => fuzzyMatch(query, file.path));
  }, [files, query]);

  const selectedFile = filteredFiles[selectedIndex];
  const handleSelect = useCallback(
    (entry: RepositoryTreeEntry) => {
      setOpen(false);
      router.push(`/${repo}/${entry.path}`); // is this right?
    },
    [repo, router, setOpen],
  );

  useEffect(() => {
    if (!open) {
      setQuery("");
      setSelectedIndex(0);
    }
  }, [open]);

  useEffect(() => {
    if (selectedIndex >= filteredFiles.length) {
      setSelectedIndex(Math.max(0, filteredFiles.length - 1));
    }
  }, [selectedIndex, filteredFiles]);

  useEffect(() => {
    if (!open) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "ArrowDown" || (e.key === "n" && e.ctrlKey)) {
        e.preventDefault();
        setSelectedIndex((prev) =>
          Math.min(prev + 1, filteredFiles.length - 1),
        );
      } else if (e.key === "ArrowUp" || (e.key === "p" && e.ctrlKey)) {
        e.preventDefault();
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
      } else if (e.key === "Enter") {
        e.preventDefault();
        if (selectedFile) {
          handleSelect(selectedFile);
        }
      } else if (e.key === "Escape") {
        e.preventDefault();
        setOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, setOpen, filteredFiles.length, selectedFile, handleSelect]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        // replicate fzf-lua's offset & positioning
        className="max-w-[80vw]! max-h-[85vh]! top-[47.75vh]! left-[51vw]! w-full h-full p-0 gap-0 flex flex-col"
        showOverlay={false}
      >
        <DialogTitle className="sr-only">File Search</DialogTitle>

        <div className="border-b border-border px-4 py-2 flex flex-row items-center gap-4 h-12 shrink-0">
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={`${repo}/`}
            className="flex-1 bg-transparent outline-none text-sm font-mono"
            autoFocus
          />
          <div className="text-xs text-muted-foreground whitespace-nowrap">
            {filteredFiles.length}/{files.length}
          </div>
        </div>

        <div className="flex flex-row flex-1 min-h-0">
          <div className="w-1/2 border-r border-border overflow-y-auto scrollbar-none">
            {filteredFiles.map((entry, index) => (
              <button
                type="button"
                key={entry.path}
                className={`flex flex-row w-full px-4 py-1 text-sm font-mono cursor-pointer ${
                  index === selectedIndex
                    ? "bg-accent text-accent-foreground"
                    : ""
                }`}
                onMouseEnter={() => setSelectedIndex(index)} // way to de-register when the modal initially opens? so default is always 0
                onClick={() => handleSelect(entry)}
              >
                {entry.path}
              </button>
            ))}
          </div>

          <div className="w-1/2 overflow-y-auto bg-muted/30">
            {selectedFile && (
              <div className="p-4">
                <div className="text-xs text-muted-foreground mb-2">
                  {selectedFile.path}
                </div>
                <pre className="text-sm font-mono whitespace-pre-wrap">
                  {getMockPreview(selectedFile)}
                </pre>
              </div>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
