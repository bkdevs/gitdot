"use client";

import { useWorkerContext } from "@/(main)/context/worker";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import Link from "@/ui/link";
import type {
    RepositoryBlobsResource,
    RepositoryPathsResource,
} from "gitdot-api";
import { use, useEffect, useMemo, useRef, useState } from "react";
import { useRepoContext } from "../../context";
import { fuzzyMatch } from "../../util";

export function RepoFileDialog({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const { paths, blobs } = useRepoContext();

  return (
    <RepoFileDialogInner
      owner={owner}
      repo={repo}
      paths={use(paths)}
      blobs={use(blobs)}
    />
  );
}

function RepoFileDialogInner({
  owner,
  repo,
  paths,
  blobs,
}: {
  owner: string;
  repo: string;
  paths: RepositoryPathsResource;
  blobs: RepositoryBlobsResource;
}) {
  const [previews, setPreviews] = useState<Map<string, string>>(new Map());
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [enableHover, setEnableHover] = useState(false);
  const [mouseMoved, setMouseMoved] = useState(false);

  const { shiki } = useWorkerContext();

  const files = paths.entries.filter((entry) => entry.path_type === "blob");

  // biome-ignore lint/correctness/useExhaustiveDependencies: preview is stable; worker queues messages internally until ready
  useEffect(() => {
    if (!shiki) return;

    const onMessage = (event: MessageEvent<{ path: string; html: string }>) => {
      setPreviews((prev) =>
        new Map(prev).set(event.data.path, event.data.html),
      );
    };

    shiki.addEventListener("message", onMessage);

    for (const blob of blobs.blobs) {
      if (blob.type !== "file") continue;
      shiki.postMessage({
        path: blob.path,
        code: blob.content,
        theme: "vitesse-light",
      });
    }

    return () => shiki.removeEventListener("message", onMessage);
  }, [shiki]);

  const initialMousePos = useRef<{ x: number; y: number } | null>(null);
  const selectedItemRef = useRef<HTMLAnchorElement | null>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "p" || e.key === "/") {
        const target = e.target as HTMLElement;
        if (target.tagName === "INPUT" || target.tagName === "TEXTAREA") {
          return;
        }
        e.preventDefault();
        setOpen(true);
      }
    };

    const handleOpenFileSearch = () => setOpen(true);

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("openFileSearch", handleOpenFileSearch);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("openFileSearch", handleOpenFileSearch);
    };
  }, []);

  const filteredFiles = useMemo(() => {
    if (!query) return files;

    return (
      files
        .map((file) => ({
          file,
          result: fuzzyMatch(query, file.path),
        }))
        .filter(({ result }) => result !== null)
        // biome-ignore lint/style/noNonNullAssertion: result is non-null after filter above
        .sort((a, b) => b.result!.score - a.result!.score)
        .map(({ file }) => file)
    );
  }, [files, query]);
  const selectedFile = filteredFiles[selectedIndex];

  useEffect(() => {
    if (!open || enableHover) return;
    const timer = setTimeout(() => setEnableHover(true), 100);
    return () => clearTimeout(timer);
  }, [open, enableHover]);

  useEffect(() => {
    if (!open || mouseMoved) return;

    const handleMouseMove = (e: MouseEvent) => {
      if (initialMousePos.current === null) {
        initialMousePos.current = { x: e.clientX, y: e.clientY };
        return;
      }

      const dx = e.clientX - initialMousePos.current.x;
      const dy = e.clientY - initialMousePos.current.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance > 5) {
        setMouseMoved(true);
      }
    };

    window.addEventListener("mousemove", handleMouseMove);
    return () => window.removeEventListener("mousemove", handleMouseMove);
  }, [open, mouseMoved]);

  useEffect(() => {
    if (!open) {
      setQuery("");
      setSelectedIndex(0);
      setEnableHover(false);
      setMouseMoved(false);
      initialMousePos.current = null;
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
          selectedItemRef.current?.click();
        }
      } else if (e.key === "Escape") {
        e.preventDefault();
        setOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, filteredFiles.length, selectedFile]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        // replicate fzf-lua's offset & positioning
        className="max-w-[80vw]! max-h-[85vh]! top-[47.75vh]! left-[51vw]! w-full h-full p-0 gap-0 flex flex-col"
        aria-describedby={undefined}
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
                <Link
                  key={entry.path}
                  href={`/${owner}/${repo}/${entry.path}`}
                  ref={index === selectedIndex ? selectedItemRef : null}
                  prefetch={false}
                  onClick={() => setOpen(false)}
                  onMouseEnter={() =>
                    enableHover && mouseMoved && setSelectedIndex(index)
                  }
                  className={`flex flex-row w-full px-4 text-sm font-mono cursor-pointer truncate ${
                    index === selectedIndex
                      ? "bg-accent text-accent-foreground"
                      : ""
                  }`}
                >
                  {entry.path}
                </Link>
              ))}
            </div>
          </div>

          <div className="w-3/5 flex flex-col text-sm scrollbar-none overflow-y-hidden">
            {selectedFile && previews.has(selectedFile.path) && (
              <div
                className="px-2 py-2"
                // biome-ignore lint/security/noDangerouslySetInnerHtml: trusted Shiki-rendered HTML
                dangerouslySetInnerHTML={{
                  __html: previews.get(selectedFile.path) ?? "",
                }}
              />
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
