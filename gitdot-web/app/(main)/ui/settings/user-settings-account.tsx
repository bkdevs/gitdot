"use client";

import { useState } from "react";
import { useUserContext } from "@/(main)/context/user";
import { cn } from "@/util";
import { UserAddEmailDialog } from "./user-add-email-dialog";
import { UserChangeNameDialog } from "./user-change-name-dialog";

export function UserSettingsAccount({
  setUserSettingsOpen,
}: {
  setUserSettingsOpen: (open: boolean) => void;
}) {
  const [changeOpen, setChangeOpen] = useState(false);
  const [addEmailOpen, setAddEmailOpen] = useState(false);

  return (
    <>
      <div className="divide-y divide-border">
        <AccountAction
          title="Change username"
          description="Pick a new handle. Your old username becomes available for anyone else to claim, links pointing to your old profile will break, and every existing git remote must be updated to the new URL."
          actionLabel="Change"
          onAction={() => setChangeOpen(true)}
        />
        <EmailsSection onAddEmail={() => setAddEmailOpen(true)} />
        <AccountAction
          title="Delete account"
          description="Permanently remove your account, repositories, and personal data. This cannot be undone."
          actionLabel="Delete"
          destructive
          onAction={() => {}}
        />
      </div>
      <UserChangeNameDialog
        open={changeOpen}
        setOpen={setChangeOpen}
        setUserSettingsOpen={setUserSettingsOpen}
      />
      <UserAddEmailDialog open={addEmailOpen} setOpen={setAddEmailOpen} />
    </>
  );
}

function EmailsSection({ onAddEmail }: { onAddEmail: () => void }) {
  const { emails } = useUserContext();

  const verified = (emails ?? [])
    .filter((e) => e.is_verified)
    .sort((a, b) => {
      if (a.is_primary !== b.is_primary) return a.is_primary ? -1 : 1;
      return 0;
    });

  return (
    <div className="p-3">
      <p className="text-sm font-medium dark:font-normal">Manage emails</p>
      <p className="text-sm text-muted-foreground">
        Manage your emails. gitdot attributes commits to the emails listed here
        — match one to your git config.
      </p>
      {verified.length > 0 && (
        <div className="mt-3">
          {verified.map((e) => (
            <div
              key={e.email}
              className="grid grid-cols-[1fr_auto] items-center gap-x-3 h-6"
            >
              <span className="text-sm truncate">{e.email}</span>
              <span className="text-xs text-muted-foreground font-mono">
                {e.is_primary ? "primary" : "verified"}
              </span>
            </div>
          ))}
        </div>
      )}
      <div className="flex justify-start mt-3">
        <button
          type="button"
          onClick={onAddEmail}
          className="text-sm underline underline-offset-2 cursor-pointer transition-colors text-muted-foreground hover:text-foreground"
        >
          Add email
        </button>
      </div>
    </div>
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
      <p className="text-sm font-medium dark:font-normal">{title}</p>
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
