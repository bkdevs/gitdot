"use client";

import { Plus } from "lucide-react";
import { useState } from "react";
import { CreateQuestionDialog } from "./create-question-dialog";

export function CreateQuestionButton({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const [open, setOpen] = useState(false);

  return (
    <>
      <button
        type="button"
        className="flex flex-row h-full items-center px-2 border-border border-l bg-primary text-xs text-primary-foreground cursor-pointer outline-0! ring-0!"
        onClick={() => setOpen(true)}
      >
        <Plus className="size-3 mr-1.5" />
        Ask question
      </button>
      <CreateQuestionDialog
        open={open}
        setOpen={setOpen}
        owner={owner}
        repo={repo}
      />
    </>
  );
}
