"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import Image from "next/image";
import { useEffect, useState, useTransition } from "react";
import { login, loginWithGithub, verifyCode } from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { useUserContext } from "@/(main)/context/user";
import { validateEmail } from "@/util";

export function AuthDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [email, setEmail] = useState("");
  const [sent, setSent] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [canSubmit, setCanSubmit] = useState(false);
  const [githubPending, setGithubPending] = useState(false);
  const [isPending, startTransition] = useTransition();
  const isTyping = useIsTyping(email);

  useEffect(() => {
    if (open) {
      setEmail("");
      setSent(false);
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
      const result = await login(null, formData);
      if ("success" in result) {
        setSent(true);
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

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[45%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
        onInteractOutside={sent ? (e) => e.preventDefault() : undefined}
        onEscapeKeyDown={sent ? (e) => e.preventDefault() : undefined}
      >
        <VisuallyHidden>
          <DialogTitle>Authenticate</DialogTitle>
        </VisuallyHidden>
        {sent ? (
          <CodeForm onCancel={() => setOpen(false)} />
        ) : (
          <EmailForm
            email={email}
            setEmail={setEmail}
            error={error}
            canSubmit={canSubmit}
            isPending={isPending}
            githubPending={githubPending}
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
  handleSubmit,
  handleGithubLogin,
}: {
  email: string;
  setEmail: (v: string) => void;
  error: string | null;
  canSubmit: boolean;
  isPending: boolean;
  githubPending: boolean;
  handleSubmit: (e: React.FormEvent) => void;
  handleGithubLogin: () => void;
}) {
  return (
    <form onSubmit={handleSubmit} className="flex flex-col text-sm" noValidate>
      <p className="px-2 py-2">Login.</p>
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
          {error && <p className="text-xs text-red-500">{error}</p>}
        </div>
        <div className="flex items-center h-full">
          <button
            type="button"
            onClick={handleGithubLogin}
            disabled={githubPending}
            className="flex items-center gap-1.5 px-2 h-full text-xs border-l border-border text-primary hover:bg-accent/50 transition-colors duration-200"
          >
            <Image src="/github-logo.svg" alt="GitHub" width={14} height={14} />
            {githubPending ? "Redirecting..." : "GitHub"}
          </button>
          <button
            type="submit"
            disabled={!canSubmit || isPending}
            className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
          >
            Submit
          </button>
        </div>
      </div>
    </form>
  );
}

function CodeForm({ onCancel }: { onCancel: () => void }) {
  const { refreshUser } = useUserContext();
  const [code, setCode] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const isValid = /^[a-zA-Z0-9]{6}$/.test(code);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    const formData = new FormData();
    formData.append("code", code);
    startTransition(async () => {
      const result = await verifyCode(null, formData);
      if ("error" in result) {
        setError(result.error);
      } else {
        refreshUser();
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
          {error && <p className="text-xs text-red-500">{error}</p>}
        </div>
        <div className="flex items-center h-full">
          <button
            type="button"
            onClick={onCancel}
            className="flex items-center px-2 h-full text-xs border-l border-border text-primary hover:bg-accent/50 transition-colors duration-200"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={!isValid || isPending}
            className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
          >
            Submit
          </button>
        </div>
      </div>
    </form>
  );
}
