"use client";

import { useActionState, useState } from "react";
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
  const [owner, setOwner] = useState("");
  const [repoName, setRepoName] = useState("");
  const [visibility, setVisibility] = useState("public");

  const [state, formAction, isPending] = useActionState(
    async (_prev: CreateRepositoryActionResult | null, formData: FormData) => {
      const result = await createRepositoryAction(formData);
      if ("repository" in result) {
        setOpen(false);
        setOwner("");
        setRepoName("");
        setVisibility("public");
      }
      return result;
    },
    null,
  );

  const isValid = owner.trim() !== "" && repoName.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <div className="px-2 py-4 border-b border-border">
          <DialogTitle>Create a new repository</DialogTitle>
        </div>
        <form action={formAction} className="relative">
          <div className="flex flex-col gap-1 p-2 border-b border-border">
            <label htmlFor="owner" className="text-xs text-muted-foreground">
              Owner
            </label>
            <input
              type="text"
              id="owner"
              name="owner"
              placeholder="username"
              value={owner}
              onChange={(e) => setOwner(e.target.value)}
              className="w-full bg-background outline-none"
              disabled={isPending}
              autoFocus
            />
          </div>
          <div className="flex flex-col gap-1 p-2 border-b border-border">
            <label
              htmlFor="repo-name"
              className="text-xs text-muted-foreground"
            >
              Repository name
            </label>
            <input
              type="text"
              id="repo-name"
              name="repo-name"
              placeholder="my-awesome-project"
              value={repoName}
              onChange={(e) => setRepoName(e.target.value)}
              className="w-full bg-background outline-none"
              disabled={isPending}
            />
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
              value={visibility}
              onChange={(e) => setVisibility(e.target.value)}
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
