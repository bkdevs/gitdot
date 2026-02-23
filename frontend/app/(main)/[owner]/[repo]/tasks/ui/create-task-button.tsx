"use client";

import { Plus } from "lucide-react";
import { useState } from "react";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import { CreateTaskDialog } from "./create-task-dialog";

export function CreateTaskButton({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const [open, setOpen] = useState(false);
  const { requireAuth } = useAuthBlocker();

  return (
    <>
      <button
        type="button"
        className="flex flex-row h-full items-center px-2 border-border border-l bg-primary text-xs text-primary-foreground hover:bg-primary/80 outline-0! ring-0!"
        onClick={() => {
          if (requireAuth()) return;
          setOpen(true);
        }}
      >
        <Plus className="size-3 mr-1.5" />
        New task
      </button>
      <CreateTaskDialog
        open={open}
        setOpen={setOpen}
        owner={owner}
        repo={repo}
      />
    </>
  );
}
