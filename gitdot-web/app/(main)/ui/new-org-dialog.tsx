"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useActionState, useEffect, useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import {
  type CreateOrganizationActionResult,
  createOrganizationAction,
} from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function NewOrgDialog() {
  const { user } = useUserContext();
  const [open, setOpen] = useState(false);
  const [orgName, setOrgName] = useState("");
  const [state, formAction, isPending] = useActionState(
    async (
      _prev: CreateOrganizationActionResult | null,
      formData: FormData,
    ) => {
      const result = await createOrganizationAction(formData);
      if ("organization" in result) {
        setOpen(false);
        setOrgName("");
      }
      return result;
    },
    null,
  );

  useEffect(() => {
    const handle = () => {
      if (user) setOpen(true);
    };
    window.addEventListener("openNewOrg", handle);
    return () => window.removeEventListener("openNewOrg", handle);
  }, [user]);

  const isValid = orgName.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>New organization</DialogTitle>
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <div className="flex flex-col gap-1 p-2 border-b border-border">
            <label htmlFor="org-name" className="text-xs text-muted-foreground">
              Name
            </label>
            <input
              type="text"
              id="org-name"
              name="org-name"
              placeholder="Organization name..."
              value={orgName}
              onChange={(e) => setOrgName(e.target.value)}
              className="w-full bg-background outline-none"
              disabled={isPending}
            />
          </div>
          {state && "error" in state && (
            <p className="text-xs text-red-500 px-3 pb-2">{state.error}</p>
          )}
          <div className="flex items-center justify-between pl-2 py-2 h-9">
            <span className="text-xs text-muted-foreground">
              Create a new organization
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
