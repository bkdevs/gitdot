"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useActionState, useState } from "react";
import { useUser } from "@/(main)/providers/user-provider";
import {
  type CreateRepositoryActionResult,
  createRepositoryAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export default function CreateRepoDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const user = useUser();
  const [repoName, setRepoName] = useState("");
  const [state, formAction, isPending] = useActionState(
    async (_prev: CreateRepositoryActionResult | null, formData: FormData) => {
      const result = await createRepositoryAction(formData);
      if ("repository" in result) {
        setOpen(false);
        setRepoName("");
      }
      return result;
    },
    null,
  );

  const isValid = repoName.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>New repository</DialogTitle>
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <div className="flex flex-col gap-1 p-2 border-b border-border">
            <label
              htmlFor="repo-name"
              className="text-xs text-muted-foreground"
            >
              Name
            </label>
            <input
              type="text"
              id="repo-name"
              name="repo-name"
              placeholder="Project name..."
              value={repoName}
              onChange={(e) => setRepoName(e.target.value)}
              className="w-full bg-background outline-none"
              disabled={isPending}
            />
          </div>
          <div className="flex flex-col gap-1 p-2 border-b border-border">
            <label htmlFor="owner" className="text-xs text-muted-foreground">
              Owner
            </label>
            <select
              id="owner"
              name="owner"
              className="w-full text-sm bg-background outline-none"
              disabled={isPending}
            >
              <option value={user?.name}>{user?.name}</option>
            </select>
          </div>
          <div className="flex flex-col gap-1 p-2 border-b border-border">
            <label
              htmlFor="visibility"
              className="text-xs text-muted-foreground"
            >
              Visibility
            </label>
            <select
              id="visibility"
              name="visibility"
              className="w-full text-sm bg-background outline-none"
              disabled={isPending}
            >
              <option value="public">Public</option>
              <option value="private">Private</option>
            </select>
          </div>
          {state && "error" in state && (
            <p className="text-xs text-red-500 px-3 pb-2">{state.error}</p>
          )}
          <div className="flex items-center justify-between pl-2 py-2 border-t border-border h-9">
            <span className="text-xs text-muted-foreground">
              Create a new repository
            </span>
            <div>
              <button
                type="reset"
                className="px-3 py-1.5 h-9 text-xs border-b border-l border-r hover:bg-accent/50"
                onClick={() => setOpen(false)}
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || isPending}
                className="px-3 py-1.5 h-9 text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isPending ? "Creating..." : "Create"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
