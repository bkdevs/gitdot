"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useRouter } from "next/navigation";
import { useActionState, useEffect, useState } from "react";
import { useUserContext } from "@/(main)/provider/user";
import { createOrganizationAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";

export function NewOrgDialog() {
  const { user } = useUserContext();
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [orgName, setOrgName] = useState("");
  const [orgReadme, setOrgReadme] = useState("");

  useEffect(() => {
    if (!open) {
      setOrgName("");
      setOrgReadme("");
    }
  }, [open]);

  const [state, formAction, isPending] = useActionState(
    createOrganizationAction,
    null,
  );

  useEffect(() => {
    if (state && "organization" in state) {
      setOpen(false);
      router.push(`/${state.organization.name}`);
    }
  }, [state, router]);

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
        className="max-w-xl min-w-xl border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>New organization</DialogTitle>
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <div className="flex">
            <div className="flex flex-col w-2/3 border-r border-border">
              <div className="group flex flex-col gap-1 p-2 border-b border-border">
                <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
                  <span className="text-foreground/40 select-none"># </span>
                  Name
                </p>
                <input
                  type="text"
                  id="org-name"
                  name="org-name"
                  placeholder="my-org"
                  value={orgName}
                  onChange={(e) => setOrgName(e.target.value)}
                  className="w-full text-sm bg-background outline-none"
                  disabled={isPending}
                />
              </div>
              <div className="group flex flex-col flex-1 gap-1 p-2 border-b border-border">
                <p className="text-xs text-muted-foreground group-focus-within:text-foreground font-mono transition-colors">
                  <span className="text-foreground/40 select-none"># </span>
                  README.md
                </p>
                <textarea
                  id="org-readme"
                  name="org-readme"
                  placeholder="who you all are and what you're building...."
                  value={orgReadme}
                  onChange={(e) => setOrgReadme(e.target.value)}
                  className="w-full flex-1 text-sm bg-background outline-none resize-none"
                  disabled={isPending}
                />
              </div>
            </div>
            <div className="flex flex-col gap-2 w-1/3 px-2 pt-2 pb-16 border-b border-border">
              <div className="pb-0">
                <h2 className="text-sm font-medium dark:font-normal">
                  New organization
                </h2>
                <p className="text-xs text-muted-foreground leading-normal">
                  A new home for your team.
                </p>
              </div>
              <p className="text-xs text-muted-foreground leading-normal">
                Organizations let you share repositories with teammates and
                manage permissions in one place.
              </p>
            </div>
          </div>
          <div className="flex items-center justify-between h-7">
            <span
              className={cn(
                "pl-2 text-xs truncate",
                state && "error" in state
                  ? "text-red-500"
                  : "text-muted-foreground",
              )}
            >
              {state && "error" in state
                ? state.error
                : "Create a new organization"}
            </span>
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
                className="flex items-center px-3 h-full text-xs bg-primary text-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed cursor-pointer"
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
