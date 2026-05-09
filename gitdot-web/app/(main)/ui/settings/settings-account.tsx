"use client";

import type { UserResource } from "gitdot-api";
import { useState } from "react";
import { cn } from "@/util";
import { ChangeUsernameDialog } from "./change-username-dialog";

export function SettingsAccount({
  setSettingsOpen,
}: {
  setSettingsOpen: (open: boolean) => void;
}) {
  const [changeOpen, setChangeOpen] = useState(false);

  return (
    <>
      <div className="divide-y divide-border">
        <AccountAction
          title="Change username"
          description="Pick a new handle. Your old username becomes available for anyone else to claim, and links pointing to your old profile will break."
          actionLabel="Change"
          onAction={() => setChangeOpen(true)}
        />
        <AccountAction
          title="Delete account"
          description="Permanently remove your account, repositories, and personal data. This cannot be undone."
          actionLabel="Delete"
          destructive
          onAction={() => {}}
        />
      </div>
      <ChangeUsernameDialog
        open={changeOpen}
        setOpen={setChangeOpen}
        setSettingsOpen={setSettingsOpen}
      />
    </>
  );
}

function AccountAction({
  title,
  description,
  actionLabel,
  destructive = false,
  onAction,
}: {
  title: string;
  description: string;
  actionLabel: string;
  destructive?: boolean;
  onAction: () => void;
}) {
  return (
    <div className="p-3">
      <p className="text-sm font-medium">{title}</p>
      <p className="text-sm text-muted-foreground">{description}</p>
      <div className="flex justify-start mt-3">
        <button
          type="button"
          onClick={onAction}
          className={cn(
            "text-sm underline underline-offset-2 cursor-pointer transition-colors",
            destructive
              ? "text-destructive hover:text-destructive/80"
              : "text-muted-foreground hover:text-foreground",
          )}
        >
          {actionLabel}
        </button>
      </div>
    </div>
  );
}
