"use client";

import { signup } from "@/actions";
import { cn, validateEmail, validateName } from "@/util";
import { useActionState, useState } from "react";

export default function SignupForm() {
  const [email, setEmail] = useState("");
  const [name, setName] = useState("");

  const [state, formAction, isPending] = useActionState(signup, null);
  const canSubmit = validateEmail(email) && validateName(name) && !isPending;

  if (state && "success" in state) {
    return (
      <div className="flex flex-col text-sm w-sm">
        <p className="pb-2">Check your email.</p>
        <p className="text-primary/60">
          We sent a verification link to {email}. Click the link to verify your
          account.
        </p>
      </div>
    );
  }

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm">
      <p className="pb-2">Sign up.</p>

      <input
        type="email"
        name="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors duration-150"
      />

      <input
        type="text"
        name="name"
        placeholder="Username"
        value={name}
        onChange={(e) => setName(e.target.value)}
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
            "underline transition-all duration-300",
            canSubmit
              ? "cursor-pointer decoration-current"
              : "text-primary/60 cursor-not-allowed decoration-transparent",
          )}
          disabled={isPending || email === "" || name === ""}
        >
          {isPending ? "Submitting..." : "Submit."}
        </button>
      </div>
    </form>
  );
}
