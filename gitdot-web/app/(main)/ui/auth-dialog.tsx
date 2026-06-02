"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { useEffect, useState, useTransition } from "react";
import { useUserContext } from "@/(main)/context/user";
import { loginWithGithub, sendCode, verifyCode } from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn, validateEmail } from "@/util";

type Step = "email" | "code";
export type AuthScreen = "login" | "signup";

export function AuthDialog() {
  const [open, setOpen] = useState(false);
  const [screen, setScreen] = useState<AuthScreen>("login");
  const [email, setEmail] = useState("");
  const [step, setStep] = useState<Step>("email");
  const [error, setError] = useState<string | null>(null);
  const [canSubmit, setCanSubmit] = useState(false);
  const [githubPending, setGithubPending] = useState(false);
  const [isPending, startTransition] = useTransition();
  const isTyping = useIsTyping(email);

  useEffect(() => {
    const handler = (e: Event) => {
      const screen = (e as CustomEvent<{ screen?: AuthScreen }>).detail?.screen;
      setScreen(screen ?? "login");
      setOpen(true);
    };
    window.addEventListener("openAuthDialog", handler);
    return () => window.removeEventListener("openAuthDialog", handler);
  }, []);

  useEffect(() => {
    if (open) {
      setEmail("");
      setStep("email");
      setError(null);
      setCanSubmit(false);
      setGithubPending(false);
    }
  }, [open]);

  useEffect(() => {
    if (!isTyping) {
      setCanSubmit(validateEmail(email) && !isPending);
    }
  }, [email, isTyping, isPending]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    const formData = new FormData();
    formData.append("email", email);
    startTransition(async () => {
      const result = await sendCode(null, formData);
      if ("success" in result) {
        setStep("code");
      } else {
        setError(result.error);
      }
    });
  };

  const handleGithubLogin = async () => {
    setGithubPending(true);
    await loginWithGithub();
    setGithubPending(false);
  };

  const blockClose = step === "code";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[45%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
        onInteractOutside={blockClose ? (e) => e.preventDefault() : undefined}
        onEscapeKeyDown={blockClose ? (e) => e.preventDefault() : undefined}
      >
        <VisuallyHidden>
          <DialogTitle>Authenticate</DialogTitle>
        </VisuallyHidden>
        {step === "code" ? (
          <CodeForm email={email} onCancel={() => setOpen(false)} />
        ) : (
          <EmailForm
            email={email}
            setEmail={setEmail}
            error={error}
            canSubmit={canSubmit}
            isPending={isPending}
            githubPending={githubPending}
            showSignup={screen === "signup"}
            toggleSignup={() =>
              setScreen((s) => (s === "signup" ? "login" : "signup"))
            }
            handleSubmit={handleSubmit}
            handleGithubLogin={handleGithubLogin}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

function EmailForm({
  email,
  setEmail,
  error,
  canSubmit,
  isPending,
  githubPending,
  showSignup,
  toggleSignup,
  handleSubmit,
  handleGithubLogin,
}: {
  email: string;
  setEmail: (v: string) => void;
  error: string | null;
  canSubmit: boolean;
  isPending: boolean;
  githubPending: boolean;
  showSignup: boolean;
  toggleSignup: () => void;
  handleSubmit: (e: React.FormEvent) => void;
  handleGithubLogin: () => void;
}) {
  return (
    <form onSubmit={handleSubmit} className="flex flex-col text-sm" noValidate>
      <p className="px-2 py-2">{showSignup ? "Signup." : "Login."}</p>
      <input
        type="email"
        name="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        className="w-full px-2 pb-2 border-b border-border ring-0 outline-0"
        disabled={isPending}
        autoFocus
      />
      <div className="flex items-center justify-between h-8">
        <div className="flex items-center px-2">
          {error ? (
            <p className="text-xs text-red-500">{error}</p>
          ) : (
            <button
              type="button"
              onClick={toggleSignup}
              className="text-xs text-muted-foreground hover:text-foreground cursor-pointer transition-colors duration-200"
            >
              {showSignup
                ? "Already have an account? Login."
                : "Don't have an account? Signup."}
            </button>
          )}
        </div>
        <div className="flex items-center h-full">
          <button
            type="button"
            onClick={handleGithubLogin}
            disabled={githubPending}
            className="flex items-center gap-1.5 px-2 h-full text-xs border-l border-border text-foreground hover:bg-accent/50 transition-colors duration-200"
          >
            <Image src="/github-logo.svg" alt="GitHub" width={14} height={14} />
            {githubPending ? "Redirecting..." : "GitHub"}
          </button>
          <button
            type="submit"
            disabled={!canSubmit || isPending}
            className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition duration-200"
          >
            Submit
          </button>
        </div>
      </div>
    </form>
  );
}

function CodeForm({
  email,
  onCancel,
}: {
  email: string;
  onCancel: () => void;
}) {
  const { refreshUser } = useUserContext();
  const router = useRouter();
  const [code, setCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const [countdown, setCountdown] = useState(30);
  const [resent, setResent] = useState(false);
  const [, startResend] = useTransition();
  const isValid = /^[a-zA-Z0-9]{6}$/.test(code);

  useEffect(() => {
    if (countdown <= 0) return;
    const timer = setTimeout(() => setCountdown((c) => c - 1), 1000);
    return () => clearTimeout(timer);
  }, [countdown]);

  const handleResend = () => {
    const formData = new FormData();
    formData.append("email", email);
    startResend(async () => {
      await sendCode(null, formData);
      setResent(true);
      setCountdown(30);
    });
  };

  const resendLabel =
    countdown > 0
      ? resent
        ? "Code resent."
        : `Resend code in ${countdown}s`
      : "Resend code";

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    const formData = new FormData();
    formData.append("email", email);
    formData.append("code", code);
    startTransition(async () => {
      const result = await verifyCode(null, formData);
      if ("error" in result) {
        setError(result.error);
      } else {
        await refreshUser();
        if (result.is_new) {
          router.push("/onboarding");
          return;
        }
        onCancel();
      }
    });
  };

  return (
    <form onSubmit={handleSubmit} className="flex flex-col text-sm" noValidate>
      <p className="px-2 py-2">Check your email — we sent a code.</p>
      <input
        type="text"
        name="code"
        placeholder="Code"
        value={code}
        onChange={(e) => setCode(e.target.value)}
        maxLength={6}
        className="w-full px-2 pb-2 border-b border-border ring-0 outline-0"
        disabled={isPending}
        autoFocus
      />
      <div className="flex items-center justify-between h-8">
        <div className="flex items-center px-2">
          {error ? (
            <p className="text-xs text-red-500">{error}</p>
          ) : (
            <button
              type="button"
              onClick={handleResend}
              disabled={countdown > 0}
              className={cn(
                "text-xs transition-colors duration-200",
                countdown > 0
                  ? "text-muted-foreground cursor-not-allowed"
                  : "text-muted-foreground hover:text-foreground underline cursor-pointer",
              )}
            >
              {resendLabel}
            </button>
          )}
        </div>
        <div className="flex items-center h-full">
          <button
            type="button"
            onClick={onCancel}
            className="flex items-center px-2 h-full text-xs border-l border-border text-foreground hover:bg-accent/50 transition-colors duration-200"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={!isValid || isPending}
            className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition duration-200"
          >
            Submit
          </button>
        </div>
      </div>
    </form>
  );
}
