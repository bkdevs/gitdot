"use client";

import { useRouter } from "next/navigation";
import { use, useCallback, useEffect, useMemo, useState } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { fuzzyMatch, getMockPreview } from "../util";
import type { JSX } from "react";

export function RepoFileDialog({
  open,
  setOpen,
  repo,
  files,
  filePreviewsPromise
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  repo: string;
  files: RepositoryTreeEntry[];
  filePreviewsPromise: Promise<Map<string, JSX.Element>>;
}) {
  const router = useRouter();
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [mouseMoved, setMouseMoved] = useState(false);
  const filePreviews = use(filePreviewsPromise);

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
    if (mouseMoved) return;

    const handleMove = () => setMouseMoved(true);
    window.addEventListener("mousemove", handleMove, { once: true });
    return () => window.removeEventListener("mousemove", handleMove);
  }, [mouseMoved]);

  useEffect(() => {
    if (!open) {
      setQuery("");
      setSelectedIndex(0);
      setMouseMoved(false);
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
      if (e.key === "u" && e.ctrlKey) {
        e.preventDefault();
        setQuery("");
      } else if (e.key === "ArrowDown" || (e.key === "n" && e.ctrlKey)) {
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
        <DialogTitle className="sr-only">File search</DialogTitle>

        <div className="flex flex-row flex-1 min-h-0">
          <div className="w-2/5 border-r border-border flex flex-col">
            <div className="border-b border-border px-4 h-9 flex flex-row items-center shrink-0">
              <div className="flex-1 flex items-center text-sm font-mono border-0 p-0 m-0 leading-normal">
                <span className="text-primary/60">{`${repo}/`}</span>
                <input
                  type="text"
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  className="flex-1 bg-transparent outline-none"
                  autoFocus
                />
              </div>
              <div className="text-xs text-muted-foreground whitespace-nowrap">
                {filteredFiles.length}/{files.length}
              </div>
            </div>
            <div className="overflow-y-auto scrollbar-none flex-1">
              {filteredFiles.map((entry, index) => (
                <button
                  type="button"
                  key={entry.path}
                  className={`flex flex-row w-full px-4 text-sm font-mono cursor-pointer truncate ${
                    index === selectedIndex
                      ? "bg-accent text-accent-foreground"
                      : ""
                  }`}
                  onMouseEnter={() => mouseMoved && setSelectedIndex(index)}
                  onClick={() => handleSelect(entry)}
                >
                  {entry.path}
                </button>
              ))}
            </div>
          </div>

          <div className="w-3/5 flex flex-col text-sm scrollbar-none overflow-y-hidden">
            {selectedFile && (
              <div className="px-2 py-2">
                {filePreviews.get(selectedFile.path)}
              </div>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
