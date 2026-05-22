"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useActionState, useEffect, useState } from "react";
import { toast } from "@/(main)/provider/toaster";
import { addOrganizationMemberAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";

export function NewMemberDialog({
  orgName,
  open,
  setOpen,
}: {
  orgName: string;
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [userName, setUserName] = useState("");
  const [roleDescription, setRoleDescription] = useState("");
  const [state, formAction, isPending] = useActionState(
    addOrganizationMemberAction.bind(null, orgName),
    null,
  );

  useEffect(() => {
    if (open) {
      setUserName("");
      setRoleDescription("");
    }
  }, [open]);

  useEffect(() => {
    if (state && "member" in state) {
      setOpen(false);
      toast.success("Member added");
    }
  }, [state, setOpen]);

  const isValid = userName.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="p-0 overflow-hidden w-[28rem] top-[45%]"
        animations
        showOverlay
      >
        <VisuallyHidden>
          <DialogTitle>Add member</DialogTitle>
        </VisuallyHidden>
        <form action={formAction}>
          <div className="group flex flex-col gap-1 p-2 border-b border-border">
            <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
              <span className="text-foreground/40 select-none"># </span>
              Username
            </p>
            <input
              type="text"
              name="user_name"
              value={userName}
              onChange={(e) => setUserName(e.target.value)}
              placeholder="username..."
              className="w-full text-sm bg-background outline-none"
              disabled={isPending}
              autoFocus
              autoComplete="off"
              spellCheck={false}
            />
          </div>
          <div className="group flex flex-col gap-1 p-2 border-b border-border">
            <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
              <span className="text-foreground/40 select-none"># </span>
              Role description
            </p>
            <textarea
              name="role_description"
              value={roleDescription}
              onChange={(e) => setRoleDescription(e.target.value)}
              placeholder="what do they do in your org?"
              className="w-full text-sm bg-background outline-none resize-none min-h-20"
              disabled={isPending}
              autoComplete="off"
              spellCheck={false}
            />
          </div>
          <div className="flex items-center justify-between h-7">
            <div className="flex items-center px-2 min-w-0">
              <p
                className={cn(
                  "text-xs truncate",
                  state && "error" in state
                    ? "text-red-500"
                    : "text-muted-foreground",
                )}
              >
                {state && "error" in state
                  ? state.error
                  : `Add member to ${orgName}`}
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
                type="submit"
                disabled={!isValid || isPending}
                className="flex items-center px-3 h-full text-xs bg-primary text-primary-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed"
              >
                {isPending ? "Adding..." : "Add"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
