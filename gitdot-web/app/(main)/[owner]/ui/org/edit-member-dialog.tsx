"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { OrganizationMemberResource } from "gitdot-api";
import { useActionState, useEffect, useState } from "react";
import { toast } from "@/(main)/provider/toaster";
import { updateOrganizationMemberAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";

export function EditMemberDialog({
  orgName,
  member,
  open,
  setOpen,
}: {
  orgName: string;
  member: OrganizationMemberResource;
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [roleDescription, setRoleDescription] = useState(
    member.role_description ?? "",
  );
  const [state, formAction, isPending] = useActionState(
    updateOrganizationMemberAction.bind(null, orgName, member.id),
    null,
  );

  useEffect(() => {
    if (open) {
      setRoleDescription(member.role_description ?? "");
    }
  }, [open, member.role_description]);

  useEffect(() => {
    if (state && "member" in state) {
      setOpen(false);
      toast.success("Member updated");
    }
  }, [state, setOpen]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="p-0 overflow-hidden w-[28rem] top-[45%]"
        animations
        showOverlay
      >
        <VisuallyHidden>
          <DialogTitle>Edit member</DialogTitle>
        </VisuallyHidden>
        <form action={formAction}>
          <div className="group flex flex-col gap-1 p-2 border-b border-border">
            <p className="text-xs text-muted-foreground font-mono">
              <span className="text-foreground/40 select-none"># </span>
              Username
            </p>
            <p className="w-full text-sm text-muted-foreground">
              {member.user_name}
            </p>
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
                  state && "error" in state
                    ? "text-red-500"
                    : "text-muted-foreground",
                )}
              >
                {state && "error" in state
                  ? state.error
                  : `Edit ${member.user_name}'s membership`}
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
                disabled={isPending}
                className="flex items-center px-3 h-full text-xs bg-primary text-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed"
              >
                {isPending ? "Saving..." : "Save"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
