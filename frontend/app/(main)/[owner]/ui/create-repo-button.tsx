"use client";

import { Plus } from "lucide-react";
import { useState } from "react";
import CreateRepoDialog from "./create-repo-dialog";

export function CreateRepoButton() {
  const [open, setOpen] = useState(false);

  return (
    <>
      <button
        type="button"
        className="flex flex-row h-full items-center px-2 border-border border-l bg-primary text-xs text-primary-foreground hover:bg-primary/80 outline-0! ring-0!"
        onClick={() => setOpen(true)}
      >
        <Plus className="size-3 mr-1.5" />
        New repo
      </button>
      <CreateRepoDialog open={open} setOpen={setOpen} />
    </>
  );
}
