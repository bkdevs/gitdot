"use client";

import { useActionState } from "react";
import {
  type CreateRepositoryActionResult,
  createRepositoryAction,
} from "@/actions";
import { Button } from "@/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/ui/dialog";
import { Input } from "@/ui/input";

export default function CreateRepoDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [state, formAction, isPending] = useActionState(
    async (_prev: CreateRepositoryActionResult | null, formData: FormData) => {
      const result = await createRepositoryAction(formData);
      return result;
    },
    null,
  );

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent animations={true}>
        <DialogTitle>Create a new repository</DialogTitle>
        <DialogDescription>
          A repository contains all project files, including the revision
          history.
        </DialogDescription>
        <form action={formAction} className="flex flex-col gap-4 mt-2">
          <div className="flex flex-col gap-2">
            <label htmlFor="owner" className="text-sm font-medium">
              Owner
            </label>
            <Input id="owner" placeholder="username" required />
          </div>
          <div className="flex flex-col gap-2">
            <label htmlFor="repo-name" className="text-sm font-medium">
              Repository name
            </label>
            <Input id="repo-name" placeholder="my-awesome-project" required />
          </div>
          <div className="flex flex-col gap-2">
            <label htmlFor="visibility" className="text-sm font-medium">
              Visibility
            </label>
            <select
              id="visibility"
              className="h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm"
            >
              <option value="public">Public</option>
              <option value="private">Private</option>
            </select>
          </div>
          {state && "error" in state && (
            <p className="text-sm text-destructive">{state.error}</p>
          )}
          <div className="flex justify-end gap-2 mt-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => setOpen(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isPending}>
              {isPending ? "Creating..." : "Create repository"}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
