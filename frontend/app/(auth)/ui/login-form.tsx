"use client";

import { useRouter } from "next/navigation";
import { useActionState, useEffect, useState } from "react";
import { type AuthActionResult, login } from "@/actions";
import { cn, validateEmail, validatePassword } from "@/util";

export default function LoginForm({ onSuccess }: { onSuccess?: () => void }) {
  const router = useRouter();

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [debouncedPassword, setDebouncedPassword] = useState("");

  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedPassword(password);
    }, 200);

    return () => clearTimeout(timer);
  }, [password]);

  const [state, formAction, isPending] = useActionState(
    async (_prevState: AuthActionResult | null, formData: FormData) => {
      const result = await login(formData);

      if (result.success) {
        if (onSuccess) {
          onSuccess();
        } else {
          router.push("/home");
        }
      }
      return result;
    },
    null,
  );

  const canSubmit =
    validateEmail(email) && validatePassword(debouncedPassword) && !isPending;

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm">
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
        className="border-border border-b ring-0 outline-0 focus:border-black transition-colors duration-150"
      />

      <div className="flex flex-row mt-2 w-full justify-end">
        <button
          type="submit"
          className={cn(
            "underline transition-all duration-300",
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
