"use client";

import { useParams, useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { RepositoryTreeEntry } from "@/lib/dto";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

function fuzzyMatch(query: string, target: string): boolean {
  const lowerQuery = query.toLowerCase();
  const lowerTarget = target.toLowerCase();
  let queryIndex = 0;

  for (
    let i = 0;
    i < lowerTarget.length && queryIndex < lowerQuery.length;
    i++
  ) {
    if (lowerTarget[i] === lowerQuery[queryIndex]) {
      queryIndex++;
    }
  }

  return queryIndex === lowerQuery.length;
}

function getMockPreview(entry: RepositoryTreeEntry): string {
  if (entry.entry_type === "tree") {
    return "// Directory\n// Contents preview will be available soon";
  }

  const ext = entry.path.split(".").pop() || "";

  if (ext === "yaml" || ext === "yml") {
    return `# ${entry.name}\n\npackages:\n  - example\n\ndependencies:\n  - typescript: ^5.0.0\n  - react: ^19.0.0`;
  }

  if (ext === "json") {
    return `{\n  "name": "${entry.name}",\n  "version": "1.0.0",\n  "description": "Mock preview"\n}`;
  }

  if (ext === "ts" || ext === "tsx" || ext === "js" || ext === "jsx") {
    return `// ${entry.name}\n\nexport default function Component() {\n  return <div>Preview coming soon</div>;\n}`;
  }

  if (ext === "md") {
    return `# ${entry.name}\n\nThis is a mock preview of the file content.\n\nActual content will be loaded soon.`;
  }

  return `// ${entry.name}\n// File preview will be available soon\n// Type: ${entry.entry_type}\n// SHA: ${entry.sha}`;
}

export function RepoFileDialog({
  folders,
  entries,
}: {
  folders: Map<string, string[]>;
  entries: Map<string, RepositoryTreeEntry>;
}) {
  const router = useRouter();
  const params = useParams();
  const slug = params.slug as string;

  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  // Get all file entries (excluding directories for now)
  const allEntries = useMemo(() => {
    return Array.from(entries.values()).filter(
      (entry) => entry.entry_type === "blob",
    );
  }, [entries]);

  // Filter entries based on fuzzy search
  const filteredEntries = useMemo(() => {
    if (!query) return allEntries;
    return allEntries.filter((entry) => fuzzyMatch(query, entry.path));
  }, [allEntries, query]);

  // Get selected entry
  const selectedEntry = filteredEntries[selectedIndex];

  // Handle file selection
  const handleSelect = useCallback(
    (entry: RepositoryTreeEntry) => {
      setOpen(false);
      router.push(`/${slug}/${entry.path}`);
    },
    [slug, router],
  );

  // Reset selection when filtered list changes
  useEffect(() => {
    setSelectedIndex(0);
  }, []);

  // Focus input when dialog opens
  useEffect(() => {
    if (open) {
      setQuery("");
      setSelectedIndex(0);
      setTimeout(() => inputRef.current?.focus(), 0);
    }
  }, [open]);

  // Keyboard navigation
  useEffect(() => {
    if (!open) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelectedIndex((prev) =>
          Math.min(prev + 1, filteredEntries.length - 1),
        );
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
      } else if (e.key === "Enter") {
        e.preventDefault();
        if (selectedEntry) {
          handleSelect(selectedEntry);
        }
      } else if (e.key === "Escape") {
        e.preventDefault();
        setOpen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, filteredEntries.length, selectedEntry, handleSelect]);

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
            placeholder="/gitdot/frontend/"
            className="flex-1 bg-transparent outline-none text-sm font-mono"
          />
          <div className="text-xs text-muted-foreground whitespace-nowrap">
            {filteredEntries.length}/{allEntries.length}
          </div>
        </div>

        <div className="flex flex-row flex-1 min-h-0">
          <div className="w-1/2 border-r border-border overflow-y-auto scrollbar-none">
            {filteredEntries.map((entry, index) => (
              <button
                type="button"
                key={entry.path}
                className={`flex flex-row w-full px-4 py-1 text-sm font-mono cursor-pointer hover:bg-accent/50 ${
                  index === selectedIndex
                    ? "bg-accent text-accent-foreground"
                    : ""
                }`}
                onMouseEnter={() => setSelectedIndex(index)}
                onClick={() => handleSelect(entry)}
              >
                {entry.path}
              </button>
            ))}
            {filteredEntries.length === 0 && (
              <div className="px-4 py-8 text-sm text-muted-foreground text-center">
                No files found
              </div>
            )}
          </div>

          <div className="w-1/2 overflow-y-auto bg-muted/30">
            {selectedEntry ? (
              <div className="p-4">
                <div className="text-xs text-muted-foreground mb-2">
                  {selectedEntry.path}
                </div>
                <pre className="text-sm font-mono whitespace-pre-wrap">
                  {getMockPreview(selectedEntry)}
                </pre>
              </div>
            ) : (
              <div className="p-4 text-sm text-muted-foreground">
                Select a file to preview
              </div>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
