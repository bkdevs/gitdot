"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useEffect, useState, useTransition } from "react";
import { toast } from "@/(main)/context/toaster";
import { updateRepositoryAction } from "@/actions/repository";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function RepoPromoteDialog({
  open,
  setOpen,
  owner,
  repo,
  onSuccess,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  owner: string;
  repo: string;
  onSuccess?: () => void;
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

  const confirmValue = `${owner}/${repo}`;
  const isValid = confirmation === confirmValue;

  function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (!isValid || isPending) return;

    setError(null);
    startTransition(async () => {
      const result = await updateRepositoryAction(owner, repo, {
        readonly: false,
      });
      if ("error" in result) {
        setError(result.error);
        return;
      }
      toast.success("Repository promoted");
      setOpen(false);
      onSuccess?.();
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
          <DialogTitle>Promote repository</DialogTitle>
        </VisuallyHidden>
        <form onSubmit={handleSubmit}>
          <div className="p-2 border-b border-border flex flex-col gap-2">
            <p className="text-sm">Promote this repository?</p>
            <p className="text-xs text-muted-foreground">
              Collaborators will be able to push directly to gitdot and new
              commits on GitHub will no longer be synced.
            </p>
            <p className="text-xs text-muted-foreground">
              Type <span className="font-medium">{confirmValue}</span> below to
              confirm.
            </p>
            {error && <p className="text-xs text-red-500 px-2 py-1">{error}</p>}
          </div>
          <div className="flex h-7">
            <input
              type="text"
              name="confirmation"
              placeholder={confirmValue}
              value={confirmation}
              onChange={(event) => setConfirmation(event.target.value)}
              className="flex-1 min-w-0 h-full px-2 text-xs leading-7 border-0 outline-none pb-1"
              disabled={isPending}
              autoFocus
            />
            <button
              type="submit"
              disabled={!isValid || isPending}
              className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed cursor-pointer"
            >
              {isPending ? "Promoting..." : "Promote"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
