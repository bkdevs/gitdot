"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import type { RunnerResource } from "gitdot-api";
import { useEffect, useState, useTransition } from "react";
import { refreshRunnerTokenAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";

export function RefreshRunnerTokenDialog({
  open,
  setOpen,
  runner,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  runner: RunnerResource;
}) {
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const [copied, setCopied] = useState(false);
  const [secondsLeft, setSecondsLeft] = useState(0);

  useEffect(() => {
    if (!token) return;

    setSecondsLeft(15);
    const interval = setInterval(() => {
      setSecondsLeft((s) => {
        if (s <= 1) {
          clearInterval(interval);
          setToken(null);
          return 0;
        }
        return s - 1;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [token]);

  useEffect(() => {
    if (!open) {
      setToken(null);
      setError(null);
      setCopied(false);
      setSecondsLeft(0);
    }
  }, [open]);

  function handleGenerateToken() {
    setError(null);
    setToken(null);

    startTransition(async () => {
      const result = await refreshRunnerTokenAction(
        runner.name,
        runner.owner_name,
      );

      if ("error" in result) {
        setError(result.error);
      } else {
        setToken(result.token);
      }
    });
  }

  function handleCopy() {
    if (!token) return;
    navigator.clipboard.writeText(token);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  }

  function handleOpenChange(open: boolean) {
    if (token) return;
    setOpen(open);
  }

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Refresh runner token</DialogTitle>
        </VisuallyHidden>

        {!token && (
          <>
            <div className="p-2">
              <p className="text-sm pb-2">Refresh runner token</p>
              <p className="text-xs text-muted-foreground">
                This will immediately invalidate the old token. Reconfigure the
                runner with <br />
                <span className="font-mono bg-muted p-0.5">
                  gitdot runner install
                </span>{" "}
                and paste the new token when prompted.
              </p>
            </div>

            {error && (
              <p className="text-xs text-red-500 px-2 py-1 border-b border-border">
                {error}
              </p>
            )}

            <div className="flex h-9 justify-end border-t border-border">
              <button
                type="button"
                className="h-full px-3 text-xs border-l border-r border-border hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed"
                onClick={() => setOpen(false)}
                disabled={isPending}
              >
                Cancel
              </button>
              <button
                type="button"
                disabled={isPending}
                onClick={handleGenerateToken}
                className="h-full px-3 text-xs hover:bg-muted text-foreground disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isPending ? "Generating..." : "Generate"}
              </button>
            </div>
          </>
        )}
        {token && (
          <div>
            <code className="block w-full border-b border-border p-2 text-xs font-mono break-all">
              {token}
            </code>
            <div className="flex items-center justify-between h-8">
              <p className="p-2 text-xs text-muted-foreground">
                This token will disappear in {secondsLeft}s.
              </p>
              <button
                type="button"
                onClick={handleCopy}
                className="px-2 py-1.5 h-full text-xs border-l border-border rounded hover:bg-muted"
              >
                {copied ? "Copied!" : "Copy"}
              </button>
            </div>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
