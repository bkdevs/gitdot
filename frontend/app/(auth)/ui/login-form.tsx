"use client";

import { login } from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { cn, validateEmail, validatePassword } from "@/util";
import { useActionState, useEffect, useState } from "react";

export default function LoginForm({ redirect }: { redirect?: string }) {
  const [state, formAction, isPending] = useActionState(login, null);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [canSubmit, setCanSubmit] = useState(false);
  const isTyping = useIsTyping(email, password);

  useEffect(() => {
    if (!isTyping) {
      setCanSubmit(
        validateEmail(email) && validatePassword(password) && !isPending,
      );
    }
  }, [email, password, isTyping, isPending]);

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm" noValidate>
      <p className="pb-2">Login.</p>

      <input
        type="email"
        name="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors duration-150"
      />

      <input
        type="password"
        name="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors duration-150"
      />
      <input type="hidden" name="redirect" value={redirect || "/home"} />

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
