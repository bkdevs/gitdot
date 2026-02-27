"use client";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import type { RunnerResource } from "gitdot-api";
import { Settings } from "lucide-react";
import { useState } from "react";
import { DeleteRunnerDialog } from "./delete-runner-dialog";
import { RefreshRunnerTokenDialog } from "./refresh-runner-token-dialog";

export function RunnerSettings({ runner }: { runner: RunnerResource }) {
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [refreshOpen, setRefreshOpen] = useState(false);

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            type="button"
            className="flex h-8 items-center justify-center bg-background pl-2 pr-1 text-xs text-foreground hover:bg-accent/50 rounded-none border-l border-border gap-1.5 outline-0! ring-0!"
          >
            <Settings className="size-3" />
            Settings
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent side="bottom" align="end">
          <DropdownMenuItem
            onSelect={() => setRefreshOpen(true) }
          >
            Refresh token
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            variant="destructive"
            onSelect={() =>  setDeleteOpen(true) }
          >
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <RefreshRunnerTokenDialog
        open={refreshOpen}
        setOpen={setRefreshOpen}
        runner={runner}
      />
      <DeleteRunnerDialog
        open={deleteOpen}
        setOpen={setDeleteOpen}
        runner={runner}
      />
    </>
  );
}
