"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { ChevronDown } from "lucide-react";
import { useActionState, useEffect, useState } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user/user-image";
import { useUserContext } from "@/(main)/context/user";
import {
  type CreateRepositoryActionResult,
  createRepositoryAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function NewRepoDialog() {
  const { user } = useUserContext();
  const [open, setOpen] = useState(false);
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

  useEffect(() => {
    const handle = () => {
      if (user) setOpen(true);
    };
    window.addEventListener("openNewRepo", handle);
    return () => window.removeEventListener("openNewRepo", handle);
  }, [user]);

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
            <p className="text-xs text-muted-foreground font-mono">
              <span className="text-foreground/40 select-none"># </span>
              Name
            </p>
            <input
              type="text"
              id="repo-name"
              name="repo-name"
              placeholder="Name..."
              value={repoName}
              onChange={(e) => setRepoName(e.target.value)}
              className="w-full text-sm bg-background outline-none"
              disabled={isPending}
            />
          </div>
          <div className="flex flex-col gap-1 p-2">
            <p className="text-xs text-muted-foreground font-mono">
              <span className="text-foreground/40 select-none"># </span>
              Description
            </p>
            <input
              type="text"
              id="repo-description"
              name="repo-description"
              placeholder="Description..."
              className="w-full text-sm bg-background outline-none"
              disabled={isPending}
            />
          </div>
          {state && "error" in state && (
            <p className="text-xs text-red-500 px-3 pb-2">{state.error}</p>
          )}
          <div className="flex items-center justify-between h-7 border-t border-border">
            <div className="flex items-center h-full text-xs text-muted-foreground">
              <span className="px-2">New repository in</span>
              <div className="relative flex items-center gap-1.5 px-2 h-full border-l border-r border-border">
                <UserImage userId={user?.id} px={14} />
                <select
                  id="owner"
                  name="owner"
                  className="appearance-none bg-transparent outline-none pr-4 cursor-pointer"
                  disabled={isPending}
                >
                  <option value={user?.name}>{user?.name}</option>
                </select>
                <ChevronDown className="size-3 absolute right-1.5 pointer-events-none" />
              </div>
              <div className="relative flex items-center px-2 h-full border-r border-border">
                <select
                  id="visibility"
                  name="visibility"
                  className="appearance-none bg-transparent outline-none pr-4 cursor-pointer"
                  disabled={isPending}
                >
                  <option value="public">Public</option>
                  <option value="private">Private</option>
                </select>
                <ChevronDown className="size-3 absolute right-1.5 pointer-events-none" />
              </div>
            </div>
            <div className="flex items-center h-full">
              <button
                type="reset"
                onClick={() => setOpen(false)}
                className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors cursor-pointer"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || isPending}
                className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed cursor-pointer"
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
