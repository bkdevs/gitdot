"use client";

import Image from "next/image";
import { useRouter } from "next/navigation";
import { useActionState, useEffect, useState } from "react";
import { loginWithGithub, sendCode, verifyCode } from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { cn, validateEmail } from "@/util";

export default function LoginForm({ redirect }: { redirect?: string }) {
  const [step, setStep] = useState<"email" | "code">("email");
  const [email, setEmail] = useState("");

  if (step === "code") {
    return <VerifyCodeForm email={email} redirect={redirect} />;
  }
  return (
    <EmailForm
      email={email}
      setEmail={setEmail}
      onSuccess={() => setStep("code")}
    />
  );
}

function EmailForm({
  email,
  setEmail,
  onSuccess,
}: {
  email: string;
  setEmail: (email: string) => void;
  onSuccess: () => void;
}) {
  const [state, formAction, isPending] = useActionState(sendCode, null);
  const [canSubmit, setCanSubmit] = useState(false);
  const [githubPending, setGithubPending] = useState(false);

  const isTyping = useIsTyping(email);

  useEffect(() => {
    if (state && "success" in state) onSuccess();
  }, [state, onSuccess]);

  useEffect(() => {
    if (!isTyping) setCanSubmit(validateEmail(email) && !isPending);
  }, [email, isTyping, isPending]);

  const handleGithubLogin = () => {
    setGithubPending(true);
    loginWithGithub();
    setGithubPending(false);
  };

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm" noValidate>
      <p className="pb-2">Login.</p>
      <input
        type="email"
        name="email"
        placeholder="Email"
        defaultValue=""
        autoFocus
        onChange={(e) => setEmail(e.target.value)}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors duration-500"
      />
      <div className="flex flex-row w-full justify-between">
        <div className="flex items-center">
          {state && "error" in state ? (
            <p className="text-red-500">{state.error}</p>
          ) : (
            <button
              type="button"
              onClick={handleGithubLogin}
              className="flex items-center text-xs text-foreground/60 hover:text-foreground transition-colors duration-150"
            >
              {githubPending ? (
                "redirecting..."
              ) : (
                <>
                  or sign in with{" "}
                  <Image
                    src="/github-logo.svg"
                    alt=""
                    width={13}
                    height={13}
                    className="mb-0.75 mx-1"
                  />{" "}
                  GitHub
                </>
              )}
            </button>
          )}
        </div>
        <button
          type="submit"
          disabled={!canSubmit}
          className={cn(
            "cursor-pointer underline transition-all duration-300 disabled:cursor-not-allowed",
            canSubmit ? "decoration-current" : "decoration-transparent",
          )}
        >
          Submit.
        </button>
      </div>
    </form>
  );
}

function VerifyCodeForm({
  email,
  redirect,
}: {
  email: string;
  redirect?: string;
}) {
  const [state, formAction, isPending] = useActionState(verifyCode, null);
  const [code, setCode] = useState("");
  const [canSubmit, setCanSubmit] = useState(false);

  const router = useRouter();
  const isTyping = useIsTyping(code);

  useEffect(() => {
    if (state && !("error" in state)) {
      router.push(
        state.is_new ? "/onboarding" : (redirect ?? `/${state.username}`),
      );
    }
  }, [state, redirect, router]);

  useEffect(() => {
    if (!isTyping) setCanSubmit(code.length === 6 && !isPending);
  }, [code, isTyping, isPending]);

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm" noValidate>
      <p className="pb-2">Check your email — we sent a code.</p>
      <input type="hidden" name="email" value={email} />
      <input
        type="text"
        name="code"
        placeholder="Code"
        defaultValue=""
        onChange={(e) => setCode(e.target.value)}
        maxLength={6}
        className="border-border border-b mb-2 ring-0 outline-0 focus:border-black"
        autoFocus
      />
      <div className="flex flex-row w-full justify-between">
        <div className="flex">
          {state && "error" in state && (
            <p className="text-red-500">{state.error}</p>
          )}
        </div>
        <button
          type="submit"
          disabled={!canSubmit}
          className={cn(
            "cursor-pointer underline transition-all duration-300 disabled:cursor-not-allowed",
            canSubmit ? "decoration-current" : "decoration-transparent",
          )}
        >
          Submit.
        </button>
      </div>
    </form>
  );
}
