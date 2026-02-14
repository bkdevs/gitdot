"use client";

import { useActionState, useEffect, useState } from "react";
import { updateUserAction, validateUsername } from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { cn } from "@/util";

export default function UsernameForm() {
  const [state, formAction, isPending] = useActionState(updateUserAction, null);
  const [username, setUsername] = useState("");
  const [usernameError, setUsernameError] = useState<string | null | undefined>(
    undefined,
  );

  const isTyping = useIsTyping(username, 200);

  useEffect(() => {
    if (!isTyping && username) {
      let stale = false;

      validateUsername(username).then((error) => {
        if (!stale) setUsernameError(error);
      });

      return () => {
        stale = true;
      };
    }
  }, [username, isTyping]);

  useEffect(() => {
    if (!username) {
      setUsernameError(undefined);
    }
  }, [username]);

  const error = usernameError || (state && "error" in state && state.error);

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm" noValidate>
      <p className="pb-2">Welcome to gitdot.</p>

      <input
        name="username"
        placeholder="Username"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors duration-150"
      />

      <input type="hidden" name="redirect" value={"/onboarding/github"} />

      <div className="flex flex-row w-full justify-between">
        <div className="flex">
          {error && <p className="text-red-600 animate-in fade-in">{error}</p>}
          {usernameError === null && (
            <p className="text-green-600 animate-in fade-in duration-300">
              Username available
            </p>
          )}
        </div>
        <button
          type="submit"
          className={cn(
            "cursor-pointer underline transition-all duration-300",
            usernameError === null
              ? "decoration-current"
              : "decoration-transparent",
            isPending ? "cursor-default" : "",
          )}
          disabled={isPending}
        >
          {isPending ? "Submitting..." : "Submit."}
        </button>
      </div>
    </form>
  );
}
