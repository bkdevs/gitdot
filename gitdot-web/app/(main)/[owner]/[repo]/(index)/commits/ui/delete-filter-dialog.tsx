"use client";

import type { RepositoryCommitFilterResource } from "gitdot-api";
import { useState, useTransition } from "react";
import { toast } from "@/(main)/context/toaster";
import { deleteRepositoryCommitFilterAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function DeleteFilterDialog({
  open,
  setOpen,
  owner,
  repo,
  filter,
  onDeleted,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  owner: string;
  repo: string;
  filter: RepositoryCommitFilterResource | null;
  onDeleted: (filterId: string) => void;
}) {
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  const close = () => {
    setOpen(false);
    setError(null);
  };

  const handleDelete = () => {
    if (!filter || isPending) return;
    startTransition(async () => {
      const result = await deleteRepositoryCommitFilterAction(
        owner,
        repo,
        filter.id,
      );
      if ("error" in result) {
        setError(result.error);
      } else {
        onDeleted(filter.id);
        setOpen(false);
        setError(null);
        toast.success("Filter deleted");
      }
    });
  };

  return (
    <Dialog
      open={open}
      onOpenChange={(next) => {
        if (!next) {
          setOpen(false);
          setError(null);
        }
      }}
    >
      <DialogContent
        animations
        showOverlay
        className="p-0 overflow-hidden w-96"
      >
        <div className="px-2 py-2 flex flex-col gap-0 pb-1">
          <DialogTitle className="text-sm font-normal text-foreground">
            Delete {filter?.name ?? "filter"}
          </DialogTitle>
          <p className="text-xs text-muted-foreground">
            Are you sure you want to delete this filter?
          </p>
        </div>
        {error && <p className="px-2 pb-1 text-xs text-red-500">{error}</p>}
        <div className="flex items-center justify-end h-7 border-t border-border">
          <button
            type="button"
            onClick={close}
            disabled={isPending}
            className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleDelete}
            disabled={!filter || isPending}
            className="flex items-center px-3 h-full text-xs text-red-500 bg-background hover:underline hover:bg-red-50 border-l border-border transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isPending ? "Deleting..." : "Delete"}
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
