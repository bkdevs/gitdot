"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { RepositoryCommitFilterResource } from "gitdot-api";
import { useEffect, useState, useTransition } from "react";
import { toast } from "@/(main)/provider/toaster";
import { createRepositoryCommitFilterAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";

export function SaveFilterDialog({
  open,
  setOpen,
  owner,
  repo,
  initialName,
  authors,
  tags,
  paths,
  onSaved,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  owner: string;
  repo: string;
  initialName: string;
  authors: string[] | undefined;
  tags: string[] | undefined;
  paths: string[] | undefined;
  onSaved: (filter: RepositoryCommitFilterResource) => void;
}) {
  const [name, setName] = useState(initialName);
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  useEffect(() => {
    if (open) {
      setName(initialName);
      setError(null);
    }
  }, [open, initialName]);

  const isValid = name.trim() !== "";

  const onConfirm = () => {
    if (!isValid || isPending) return;
    startTransition(async () => {
      const result = await createRepositoryCommitFilterAction(owner, repo, {
        name: name.trim(),
        authors,
        tags,
        paths,
      });
      if ("filter" in result) {
        onSaved(result.filter);
        setOpen(false);
        toast.success("Filter saved");
      } else {
        setError(result.error);
      }
    });
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="p-0 overflow-hidden w-[28rem] top-[45%]"
        animations
        showOverlay
      >
        <VisuallyHidden>
          <DialogTitle>Save filter</DialogTitle>
        </VisuallyHidden>
        <div>
          <div className="group flex flex-col gap-1 p-2 border-b border-border">
            <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
              <span className="text-foreground/40 select-none"># </span>
              Filter name
            </p>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  e.preventDefault();
                  onConfirm();
                }
              }}
              placeholder="name your filter..."
              className="w-full text-sm bg-background outline-none"
              disabled={isPending}
              autoFocus
              autoComplete="off"
              spellCheck={false}
            />
          </div>
          <div className="flex items-center justify-between h-7">
            <div className="flex items-center px-2 min-w-0">
              <p
                className={cn(
                  "text-xs truncate",
                  error ? "text-red-500" : "text-muted-foreground",
                )}
              >
                {error ?? "Save filter for this repository"}
              </p>
            </div>
            <div className="flex items-center h-full">
              <button
                type="button"
                onClick={() => setOpen(false)}
                className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={onConfirm}
                disabled={!isValid || isPending}
                className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed"
              >
                {isPending ? "Saving..." : "Save"}
              </button>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
