"use client";

import { useActionState } from "react";
import { type ConnectSlackResult, connectSlack } from "@/actions";
import { cn } from "@/util";

export default function ConnectSlackForm({ state }: { state?: string }) {
  const [result, formAction, isPending] = useActionState(
    async (_prev: ConnectSlackResult | null): Promise<ConnectSlackResult> => {
      return await connectSlack(state);
    },
    null,
  );

  const canSubmit = !!state && !isPending;

  if (result && "success" in result) {
    return (
      <div className="flex flex-col text-sm w-sm">
        <p>Slack connected.</p>
      </div>
    );
  }

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm">
      <p className="pb-2">
        Do you want to connect your gitdot account to your Slack account?
      </p>

      <div className="flex flex-row mt-2 w-full justify-end">
        <button
          type="submit"
          className={cn(
            "underline transition-all duration-300",
            canSubmit
              ? "cursor-pointer decoration-current"
              : "text-primary/60 cursor-not-allowed decoration-transparent",
          )}
          disabled={!canSubmit}
        >
          {isPending ? "Connecting..." : "Connect."}
        </button>
      </div>

      {result && "error" in result && (
        <p className="text-red-500">{result.error}</p>
      )}
    </form>
  );
}
