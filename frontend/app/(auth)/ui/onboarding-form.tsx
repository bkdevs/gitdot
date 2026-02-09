"use client";

import { useEffect, useActionState, useState, useRef } from "react";
import { signup } from "@/actions";
import { cn, validateEmail, validateUsername } from "@/util";
import { useIsTyping } from "@/hooks/use-is-typing";

export default function OnboardingForm() {
  const [state, formAction, isPending] = useActionState(signup, null);
  const [username, setUsername] = useState("");
  const [canSubmit, setCanSubmit] = useState(false);
  const isTyping = useIsTyping(username);

  useEffect(() => {
    if (!isTyping) {
      setCanSubmit(validateUsername(username) && !isPending);
    }
  }, [username, isTyping, isPending]);

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

      <div className="flex flex-row w-full justify-between">
        <div className="flex">
          {state && "error" in state && (
            <p className="text-red-500">{state.error}</p>
          )}
        </div>
        <button
          type="submit"
          className={cn(
            "cursor-pointer underline transition-all duration-300",
            canSubmit ? "decoration-current" : "decoration-transparent",
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
