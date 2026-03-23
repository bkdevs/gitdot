"use client";

import { useState } from "react";
import { DeleteRepositoryDialog } from "./delete-repository-dialog";

export function RepositorySettingsGeneral({
  owner,
  repo,
}: {
  owner: string;
  repo: string;
}) {
  const [open, setOpen] = useState(false);

  return (
    <div className="flex flex-col w-full">
      <div className="flex items-center border-b px-2 h-9">
        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          General
        </h3>
      </div>
      <div className="flex items-center justify-between px-2 py-2 border-b h-9">
        <div className="flex flex-col">
          <span className="text-sm">Delete repository</span>
        </div>
        <button
          type="button"
          className="text-xs text-destructive hover:bg-muted border-l border-border flex items-center h-9 pl-2 cursor-pointer"
          onClick={() => setOpen(true)}
        >
          Delete
        </button>
      </div>
      <DeleteRepositoryDialog
        open={open}
        setOpen={setOpen}
        owner={owner}
        repo={repo}
      />
    </div>
  );
}
