"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useState } from "react";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function NewFilterDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [name, setName] = useState("");
  const [authors, setAuthors] = useState("");
  const [tags, setTags] = useState("");
  const [includedPaths, setIncludedPaths] = useState("");
  const [excludedPaths, setExcludedPaths] = useState("");

  function handleCreate() {
    setOpen(false);
    setName("");
    setAuthors("");
    setTags("");
    setIncludedPaths("");
    setExcludedPaths("");
  }

  function handleCancel() {
    setOpen(false);
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle title="New filter" />
        </VisuallyHidden>
        <div>
          <label className="border-b border-border block">
            <span className="text-xs text-muted-foreground px-2 pt-2 block">
              Name
            </span>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="backend"
              className="w-full px-2 pb-2 text-sm bg-background outline-none"
              autoFocus
            />
          </label>
          <label className="border-b border-border block">
            <span className="text-xs text-muted-foreground px-2 pt-2 block">
              Authors
            </span>
            <input
              type="text"
              value={authors}
              onChange={(e) => setAuthors(e.target.value)}
              placeholder="alice, bob"
              className="w-full px-2 pb-2 text-sm bg-background outline-none"
            />
          </label>
          <label className="border-b border-border block">
            <span className="text-xs text-muted-foreground px-2 pt-2 block">
              Tags
            </span>
            <input
              type="text"
              value={tags}
              onChange={(e) => setTags(e.target.value)}
              placeholder="feat:"
              className="w-full px-2 pb-2 text-sm bg-background outline-none"
            />
          </label>
          <label className="border-b border-border block">
            <span className="text-xs text-muted-foreground px-2 pt-2 block">
              Included paths
            </span>
            <input
              type="text"
              value={includedPaths}
              onChange={(e) => setIncludedPaths(e.target.value)}
              placeholder="src/, lib/"
              className="w-full px-2 pb-2 text-sm bg-background outline-none"
            />
          </label>
          <label className="border-b border-border block">
            <span className="text-xs text-muted-foreground px-2 pt-2 block">
              Excluded paths
            </span>
            <input
              type="text"
              value={excludedPaths}
              onChange={(e) => setExcludedPaths(e.target.value)}
              placeholder="*.lock, *.sum"
              className="w-full px-2 pb-2 text-sm bg-background outline-none"
            />
          </label>
          <div className="flex items-center justify-end h-9">
            <button
              type="button"
              className="px-3 py-1.5 h-9 text-xs border-l border-r hover:bg-accent/50"
              onClick={handleCancel}
            >
              Cancel
            </button>
            <button
              type="button"
              disabled={name.trim() === ""}
              className="px-3 py-1.5 h-9 text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={handleCreate}
            >
              Create
            </button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
