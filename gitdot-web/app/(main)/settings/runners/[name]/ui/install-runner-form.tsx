"use client";

import type { RunnerResource } from "gitdot-api";
import { useEffect, useState, useTransition } from "react";
import { refreshRunnerTokenAction } from "@/actions";

export function InstallRunnerForm({
  runner,
  ownerName,
}: {
  runner: RunnerResource;
  ownerName: string;
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

  function handleGenerateToken() {
    setError(null);
    setToken(null);
    startTransition(async () => {
      const result = await refreshRunnerTokenAction(runner.name, ownerName);
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

  return (
    <div className="flex p-4">
      <div className="min-w-0 flex-1 max-w-3xl">
        <h1 className="text-lg font-medium border-b border-border pb-2 mb-6">
          Install runner
        </h1>

        <div className="space-y-6">
          <div className="flex gap-3">
            <span className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-medium">
              1
            </span>
            <div>
              <p className="text-sm font-medium">
                Install gitdot runner on your machine
              </p>
              <pre className="mt-2 rounded border border-border bg-muted px-3 py-2 text-xs font-mono">
                curl -fsSL https://gitdot.io/install-cli-runner.sh | sh
              </pre>
            </div>
          </div>

          <div className="flex gap-3">
            <span className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-medium">
              2
            </span>
            <div>
              <p className="text-sm font-medium">Run the installer</p>
              <pre className="mt-2 rounded border border-border bg-muted px-3 py-2 text-xs font-mono">
                gitdot runner install
              </pre>
            </div>
          </div>

          <div className="flex gap-3">
            <span className="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-medium">
              3
            </span>
            <div>
              <p className="text-sm font-medium">
                Generate a token and configure your runner
              </p>
              <p className="mt-1 text-xs text-muted-foreground">
                Click the button below to generate a registration token, then
                paste it when prompted by the CLI.
              </p>

              {!token && (
                <button
                  type="button"
                  onClick={handleGenerateToken}
                  disabled={isPending}
                  className="mt-3 px-3 py-2 text-sm bg-primary text-primary-foreground rounded disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isPending ? "Generating..." : "Generate new token"}
                </button>
              )}

              {error && <p className="mt-2 text-sm text-red-500">{error}</p>}

              {token && (
                <div className="mt-3 space-y-2">
                  <div className="flex items-center gap-2">
                    <code className="flex-1 rounded border border-border bg-muted px-3 py-2 text-xs font-mono break-all">
                      {token}
                    </code>
                    <button
                      type="button"
                      onClick={handleCopy}
                      className="shrink-0 px-2 py-2 text-xs border border-border rounded hover:bg-muted"
                    >
                      {copied ? "Copied!" : "Copy"}
                    </button>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    This token will disappear in {secondsLeft}s. Run{" "}
                    <code className="bg-muted px-1 rounded">
                      gitdot runner install
                    </code>{" "}
                    and paste the token when prompted.
                  </p>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
      <aside className="w-72 shrink-0 hidden md:block border-l border-border ml-6 pl-6">
        <div className="text-sm text-muted-foreground space-y-3">
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua.
          </p>
          <p>
            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
            nisi ut aliquip ex ea commodo consequat.
          </p>
        </div>
      </aside>
    </div>
  );
}
