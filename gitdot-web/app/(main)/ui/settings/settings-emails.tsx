"use client";

import { useState } from "react";
import { useUserContext } from "@/(main)/provider/user";
import { addUserEmailAction, verifyUserEmailAction } from "@/actions";

export function SettingsEmails() {
  const { emails, refreshUser } = useUserContext();
  const [draft, setDraft] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const verified = (emails ?? [])
    .filter((e) => e.is_verified)
    .sort((a, b) => {
      if (a.is_primary !== b.is_primary) return a.is_primary ? -1 : 1;
      return 0;
    });
  const pending = (emails ?? []).filter((e) => !e.is_verified);

  async function commitDraft() {
    const value = draft?.trim() ?? "";
    if (!value) {
      setDraft(null);
      setError(null);
      return;
    }
    if (submitting) return;
    setSubmitting(true);
    const formData = new FormData();
    formData.set("email", value);
    const result = await addUserEmailAction(null, formData);
    setSubmitting(false);
    if ("error" in result) {
      setError(result.error);
      return;
    }
    setError(null);
    setDraft(null);
    await refreshUser();
  }

  function cancelDraft() {
    setDraft(null);
    setError(null);
  }

  return (
    <div className="p-3">
      <p className="text-sm font-medium dark:font-normal">Emails</p>
      <p className="text-sm text-muted-foreground">
        Verified emails attribute git commits to you when you push from any
        machine. Add the addresses you author commits under so they show up on
        your profile.
      </p>
      {verified.length > 0 && (
        <div className="mt-3 divide-y divide-border">
          {verified.map((e) => (
            <VerifiedRow
              key={e.email}
              email={e.email}
              isPrimary={e.is_primary}
            />
          ))}
        </div>
      )}
      <div className="mt-4 space-y-2">
        {pending.map((e) => (
          <PendingRow key={e.email} email={e.email} onVerified={refreshUser} />
        ))}
        {draft !== null ? (
          <div>
            <input
              value={draft}
              onChange={(ev) => {
                setDraft(ev.target.value);
                if (error) setError(null);
              }}
              onKeyDown={(ev) => {
                if (ev.key === "Enter") {
                  ev.stopPropagation();
                  commitDraft();
                } else if (ev.key === "Escape") {
                  ev.stopPropagation();
                  cancelDraft();
                }
              }}
              onBlur={commitDraft}
              autoFocus
              disabled={submitting}
              className="h-5 text-sm bg-transparent border-b border-border outline-none w-full placeholder:text-muted-foreground/40 transition-colors focus:border-foreground"
              placeholder="you@another-domain.com"
            />
            {error && (
              <span className="block text-xs text-destructive/80 mt-1">
                {error}
              </span>
            )}
          </div>
        ) : (
          <button
            type="button"
            onClick={() => setDraft("")}
            className="h-5 text-xs text-muted-foreground/60 hover:text-muted-foreground transition-colors cursor-pointer block w-full text-left"
          >
            new email
          </button>
        )}
      </div>
    </div>
  );
}

function VerifiedRow({
  email,
  isPrimary,
}: {
  email: string;
  isPrimary: boolean;
}) {
  return (
    <div className="grid grid-cols-[1fr_auto] items-center gap-x-3 py-1 h-7">
      <span className="text-sm truncate">{email}</span>
      <span className="text-xs text-muted-foreground font-mono">
        {isPrimary ? "primary" : "verified"}
      </span>
    </div>
  );
}

function PendingRow({
  email,
  onVerified,
}: {
  email: string;
  onVerified: () => Promise<void>;
}) {
  const [code, setCode] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [resentAt, setResentAt] = useState<number | null>(null);

  async function submitCode() {
    const trimmed = code.trim();
    if (!trimmed || submitting) return;
    setSubmitting(true);
    const result = await verifyUserEmailAction(email, trimmed);
    setSubmitting(false);
    if ("error" in result) {
      setError(result.error);
      return;
    }
    setError(null);
    setCode("");
    await onVerified();
  }

  async function resend() {
    if (submitting) return;
    setSubmitting(true);
    const formData = new FormData();
    formData.set("email", email);
    const result = await addUserEmailAction(null, formData);
    setSubmitting(false);
    if ("error" in result) {
      setError(result.error);
      return;
    }
    setError(null);
    setResentAt(Date.now());
  }

  return (
    <div>
      <div className="grid grid-cols-[1fr_auto] items-center gap-x-3 h-5">
        <span className="text-sm truncate text-muted-foreground">{email}</span>
        <div className="flex items-center gap-2 text-xs font-mono">
          <input
            value={code}
            onChange={(ev) => {
              setCode(ev.target.value.toUpperCase());
              if (error) setError(null);
            }}
            onKeyDown={(ev) => {
              if (ev.key === "Enter") {
                ev.stopPropagation();
                submitCode();
              }
            }}
            disabled={submitting}
            placeholder="code"
            maxLength={8}
            className="h-5 w-24 text-xs bg-transparent border-b border-border outline-none placeholder:text-muted-foreground/40 transition-colors focus:border-foreground uppercase tracking-widest"
          />
          <button
            type="button"
            onClick={resend}
            disabled={submitting}
            className="text-muted-foreground/60 hover:text-muted-foreground transition-colors cursor-pointer disabled:cursor-not-allowed"
          >
            resend
          </button>
        </div>
      </div>
      {error && (
        <span className="block text-xs text-destructive/80 mt-1">{error}</span>
      )}
      {!error && resentAt && (
        <span className="block text-xs text-muted-foreground/60 mt-1">
          code sent
        </span>
      )}
    </div>
  );
}
