"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { RunnerResource } from "gitdot-api";
import { useEffect, useState, useTransition } from "react";
import { deleteRunnerAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function DeleteRunnerDialog({
  open,
  setOpen,
  runner,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  runner: RunnerResource;
}) {
  const [confirmation, setConfirmation] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  useEffect(() => {
    if (open) {
      setConfirmation("");
      setError(null);
    }
  }, [open]);

  const isValid = confirmation.trim() === runner.name;

  function handleSubmit(event: React.SubmitEvent) {
    event.preventDefault();
    if (!isValid || isPending) return;

    setError(null);
    startTransition(async () => {
      await deleteRunnerAction(runner.owner_name, runner.name);
    });
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Delete runner</DialogTitle>
        </VisuallyHidden>
        <form onSubmit={handleSubmit}>
          <div className="p-2 border-b border-border">
            <p className="text-sm pb-2">
              Are you sure you want to delete this runner?
            </p>
            <p className="text-xs text-muted-foreground">
              This action cannot be undone. To confirm, type{" "}
              <span className="font-medium">{runner.name}</span> below.
            </p>
          </div>
          <input
            type="text"
            name="confirmation"
            placeholder={runner.name}
            value={confirmation}
            onChange={(event) => setConfirmation(event.target.value)}
            className="w-full p-2 text-sm bg-background outline-none border-b border-border"
            disabled={isPending}
            autoFocus
          />
          {error && (
            <p className="text-xs text-red-500 px-2 py-1 border-b border-border">
              {error}
            </p>
          )}
          <div className="flex h-9 justify-end">
            <button
              type="button"
              className="h-full px-3 text-xs border-l border-r border-border hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => setOpen(false)}
              disabled={isPending}
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={!isValid || isPending}
              className="h-full px-3 text-xs hover:bg-muted text-red-600 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? "Deleting..." : "Delete"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
