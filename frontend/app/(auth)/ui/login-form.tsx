"use client";

import Image from "next/image";
import { useActionState, useEffect, useState } from "react";
import { login, loginWithGithub } from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { cn, validateEmail } from "@/util";

export default function LoginForm({ redirect }: { redirect?: string }) {
  const [state, formAction, isPending] = useActionState(login, null);
  const [email, setEmail] = useState("");
  const [canSubmit, setCanSubmit] = useState(false);
  const [githubPending, setGithubPending] = useState(false);
  const isTyping = useIsTyping(email);

  useEffect(() => {
    if (!isTyping) {
      setCanSubmit(validateEmail(email) && !isPending);
    }
  }, [email, isTyping, isPending]);

  const handleGithubLogin = async () => {
    setGithubPending(true);
    await loginWithGithub();
    setGithubPending(false);
  };

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

      {redirect && <input type="hidden" name="redirect" value={redirect} />}

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

      <div className="flex items-center gap-2 mt-3 mb-3">
        <div className="flex-1 border-t border-border" />
        <span className="text-xs text-primary/40">or</span>
        <div className="flex-1 border-t border-border" />
      </div>

      <button
        type="button"
        onClick={handleGithubLogin}
        disabled={githubPending}
        className="flex items-center justify-center gap-2 border border-border py-1.5 cursor-pointer hover:bg-gray-50 transition-colors duration-150"
      >
        <Image src="/github-logo.svg" alt="GitHub" width={16} height={16} />
        {githubPending ? "Redirecting..." : "Continue with GitHub"}
      </button>
    </form>
  );
}
