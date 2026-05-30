"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useEffect, useState, useTransition } from "react";
import { toast } from "@/(main)/context/toaster";
import { useUserContext } from "@/(main)/context/user";
import { addUserEmailAction, verifyUserEmailAction } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";

type Step = "email" | "code";

export function UserAddEmailDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const { refreshUser } = useUserContext();
  const [step, setStep] = useState<Step>("email");
  const [email, setEmail] = useState("");
  const [code, setCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const [countdown, setCountdown] = useState(30);
  const [resent, setResent] = useState(false);
  const [, startResend] = useTransition();

  useEffect(() => {
    if (!open) return;
    setError(null);
    setCode("");
    setEmail("");
    setStep("email");
    setCountdown(30);
    setResent(false);
  }, [open]);

  useEffect(() => {
    if (step !== "code" || countdown <= 0) return;
    const timer = setTimeout(() => setCountdown((c) => c - 1), 1000);
    return () => clearTimeout(timer);
  }, [step, countdown]);

  function handleResend() {
    const formData = new FormData();
    formData.set("email", email);
    startResend(async () => {
      await addUserEmailAction(null, formData);
      setResent(true);
      setCountdown(30);
    });
  }

  const resendLabel =
    countdown > 0
      ? resent
        ? "Code resent."
        : `Resend code in ${countdown}s`
      : "Resend code";

  const blockClose = step === "code";

  function handleEmailSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (isPending) return;
    setError(null);
    const formData = new FormData();
    formData.set("email", email);
    startTransition(async () => {
      const result = await addUserEmailAction(null, formData);
      if ("error" in result) {
        setError(result.error);
        return;
      }
      setStep("code");
      setCountdown(30);
      setResent(false);
    });
  }

  function handleCodeSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (isPending) return;
    setError(null);
    startTransition(async () => {
      const result = await verifyUserEmailAction(email, code);
      if ("error" in result) {
        setError(result.error);
        return;
      }
      await refreshUser();
      setOpen(false);
      toast.success("Email verified");
    });
  }

  const emailValid = email.trim().length > 0;
  const codeValid = code.trim().length > 0;

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
          <DialogTitle>Add email</DialogTitle>
        </VisuallyHidden>
        {step === "email" ? (
          <form
            onSubmit={handleEmailSubmit}
            className="flex flex-col text-sm"
            noValidate
          >
            <p className="px-2 py-2">Add an email to your account.</p>
            <input
              type="email"
              name="email"
              placeholder="Email"
              value={email}
              onChange={(ev) => {
                setEmail(ev.target.value);
                if (error) setError(null);
              }}
              className="w-full px-2 pb-2 border-b border-border ring-0 outline-0"
              disabled={isPending}
              autoFocus
              autoComplete="off"
              spellCheck={false}
            />
            <div className="flex items-center justify-between h-8">
              <div className="flex items-center px-2">
                {error && <p className="text-xs text-red-500">{error}</p>}
              </div>
              <div className="flex items-center h-full">
                <button
                  type="button"
                  onClick={() => setOpen(false)}
                  className="flex items-center px-2 h-full text-xs border-l border-border text-foreground hover:bg-accent/50 transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={!emailValid || isPending}
                  className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition duration-200"
                >
                  Submit
                </button>
              </div>
            </div>
          </form>
        ) : (
          <form
            onSubmit={handleCodeSubmit}
            className="flex flex-col text-sm"
            noValidate
          >
            <p className="px-2 py-2">Check your email — we sent a code.</p>
            <input
              type="text"
              name="code"
              placeholder="Code"
              value={code}
              onChange={(ev) => {
                setCode(ev.target.value);
                if (error) setError(null);
              }}
              maxLength={8}
              className="w-full px-2 pb-2 border-b border-border ring-0 outline-0"
              disabled={isPending}
              autoFocus
              autoComplete="off"
              spellCheck={false}
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
                  onClick={() => setOpen(false)}
                  disabled={isPending}
                  className="flex items-center px-2 h-full text-xs border-l border-border text-foreground hover:bg-accent/50 transition-colors duration-200 disabled:cursor-not-allowed"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={!codeValid || isPending}
                  className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition duration-200"
                >
                  Submit
                </button>
              </div>
            </div>
          </form>
        )}
      </DialogContent>
    </Dialog>
  );
}
