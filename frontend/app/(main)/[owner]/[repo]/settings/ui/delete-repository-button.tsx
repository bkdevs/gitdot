"use client";

import { useState, useTransition } from "react";

import { Button } from "@/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/ui/dialog";
import { deleteRepositoryAction } from "@/actions";

export function DeleteRepositoryButton({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const [open, setOpen] = useState(false);
  const [confirmText, setConfirmText] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  const confirmValue = `${owner}/${repo}`;
  const isConfirmed = confirmText === confirmValue;

  function handleDelete() {
    setError(null);
    startTransition(async () => {
      const result = await deleteRepositoryAction(owner, repo);
      if ("error" in result) {
        setError(result.error);
      }
    });
  }

  return (
    <>
      <Button variant="destructive" size="sm" onClick={() => setOpen(true)}>
        Delete this repository
      </Button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogTitle>Delete repository</DialogTitle>
          <DialogDescription>
            This action cannot be undone. This will permanently delete the{" "}
            <strong>{confirmValue}</strong> repository and all of its data.
          </DialogDescription>
          <p className="text-sm mt-2">
            Please type <strong>{confirmValue}</strong> to confirm.
          </p>
          <input
            type="text"
            value={confirmText}
            onChange={(e) => setConfirmText(e.target.value)}
            placeholder={confirmValue}
            className="w-full border px-2 py-1.5 text-sm bg-background"
            autoComplete="off"
          />
          {error && <p className="text-sm text-destructive">{error}</p>}
          <div className="flex items-center justify-end gap-2 mt-2">
            <DialogClose asChild>
              <Button variant="outline" size="sm">
                Cancel
              </Button>
            </DialogClose>
            <Button
              variant="destructive"
              size="sm"
              disabled={!isConfirmed || isPending}
              onClick={handleDelete}
            >
              {isPending ? "Deleting..." : "Delete this repository"}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
