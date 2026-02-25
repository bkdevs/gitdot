"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useActionState, useState } from "react";
import { type CreateBuildActionResult, createBuildAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

const TRIGGERS = [
  { value: "push_to_main", label: "Push to main" },
  { value: "pull_request", label: "Pull request" },
] as const;

export function CreateBuildDialog({
  open,
  setOpen,
  owner,
  repo,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  owner: string;
  repo: string;
}) {
  const [commitSha, setCommitSha] = useState("");
  const [trigger, setTrigger] = useState<"push_to_main" | "pull_request">(
    "push_to_main",
  );

  const createBuild = createBuildAction.bind(null, owner, repo);
  const [state, formAction, isPending] = useActionState(
    async (_prev: CreateBuildActionResult | null, formData: FormData) => {
      const result = await createBuild(formData);
      if ("build" in result) {
        setOpen(false);
        setCommitSha("");
        setTrigger("push_to_main");
      }
      return result;
    },
    null,
  );

  const isValid = commitSha.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-2xl min-w-2xl border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle title={"New build"} />
        </VisuallyHidden>
        <form action={formAction} className="relative">
          <input
            type="text"
            name="commit_sha"
            placeholder="Commit SHA..."
            value={commitSha}
            onChange={(e) => setCommitSha(e.target.value)}
            className="w-full p-2 text-sm font-mono bg-background outline-none border-b border-border"
            disabled={isPending}
            autoFocus
          />
          <div className="flex flex-row border-b border-border">
            {TRIGGERS.map((t) => (
              <label
                key={t.value}
                className="flex flex-row items-center gap-2 px-3 py-2 text-sm cursor-pointer hover:bg-accent/50 select-none"
              >
                <input
                  type="radio"
                  name="trigger"
                  value={t.value}
                  checked={trigger === t.value}
                  onChange={() => setTrigger(t.value)}
                  disabled={isPending}
                  className="accent-primary"
                />
                {t.label}
              </label>
            ))}
          </div>
          {state && "error" in state && (
            <p className="text-xs text-red-500 px-3 pb-2">{state.error}</p>
          )}
          <div className="flex items-center justify-between pl-2 py-2 border-t border-border h-9">
            <span className="text-xs text-muted-foreground">
              Trigger a build in{" "}
              <span className="text-foreground">
                {owner}/{repo}
              </span>
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
                {isPending ? "Creating..." : "Run"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
