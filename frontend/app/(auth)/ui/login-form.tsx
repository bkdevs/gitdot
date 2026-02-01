"use client";

import { useActionState, useState } from "react";
import { type AuthActionResult, login } from "@/actions";
import { cn, validateEmail, validatePassword } from "@/util";

export default function LoginForm() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const [state, formAction, isPending] = useActionState(
    async (_prevState: AuthActionResult | null, formData: FormData) => {
      return await login(formData);
    },
    null,
  );

  const canSubmit =
    validateEmail(email) && validatePassword(password) && !isPending;

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm">
      <p className="pb-2">Login.</p>

      <input
        type="email"
        name="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors"
      />

      <input
        type="password"
        name="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        className="border-border border-b ring-0 outline-0 focus:border-black transition-colors"
      />

      <div className="flex flex-row mt-4 w-full justify-end">
        <button
          type="submit"
          className={cn(
            "underline transition-all duration-500",
            canSubmit
              ? "cursor-pointer decoration-current"
              : "text-primary/60 cursor-not-allowed decoration-transparent",
          )}
          disabled={isPending || email === "" || password === ""}
        >
          {isPending ? "Submitting..." : "Submit."}
        </button>
      </div>

      {state && "error" in state && (
        <p className="text-red-500">{state.error}</p>
      )}
    </form>
  );
}
