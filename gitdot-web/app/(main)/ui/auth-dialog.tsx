"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import Image from "next/image";
import { useEffect, useRef, useState, useTransition } from "react";
import { UserImage } from "@/(main)/[owner]/ui/user-image";
import { useUserContext } from "@/(main)/context/user";
import {
  loginWithGithub,
  sendCode,
  updateUserAction,
  uploadUserImageAction,
  validateUsername,
  verifyCode,
} from "@/actions";
import { useIsTyping } from "@/hooks/use-is-typing";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/ui/tooltip";
import { validateEmail } from "@/util";

type Step = "email" | "code" | "welcome";

export function AuthDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [email, setEmail] = useState("");
  const [step, setStep] = useState<Step>("email");
  const [error, setError] = useState<string | null>(null);
  const [canSubmit, setCanSubmit] = useState(false);
  const [githubPending, setGithubPending] = useState(false);
  const [isPending, startTransition] = useTransition();
  const isTyping = useIsTyping(email);

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

  const blockClose = step === "code" || step === "welcome";

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
        {step === "welcome" ? (
          <WelcomeForm open={open} onDone={() => setOpen(false)} />
        ) : step === "code" ? (
          <CodeForm
            onCancel={() => setOpen(false)}
            onWelcome={() => setStep("welcome")}
          />
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
  onCancel,
  onWelcome,
}: {
  onCancel: () => void;
  onWelcome: () => void;
}) {
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
      } else if ("is_new" in result) {
        refreshUser();
        onWelcome();
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
            className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition duration-200"
          >
            Submit
          </button>
        </div>
      </div>
    </form>
  );
}

function WelcomeForm({ onDone }: { open: boolean; onDone: () => void }) {
  const { user, refreshUser } = useUserContext();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [username, setUsername] = useState("");
  const [usernameError, setUsernameError] = useState<string | null | undefined>(
    undefined,
  );
  const [uploading, setUploading] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const isTyping = useIsTyping(username, 200);

  useEffect(() => {
    if (!isTyping && username) {
      let stale = false;
      validateUsername(username).then((error) => {
        if (!stale) setUsernameError(error);
      });
      return () => {
        stale = true;
      };
    }
  }, [username, isTyping]);

  useEffect(() => {
    if (!username) setUsernameError(undefined);
  }, [username]);

  async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;
    setUploadError(null);
    setUploading(true);
    const result = await uploadUserImageAction(file);
    setUploading(false);
    if ("error" in result) {
      setUploadError(result.error);
    } else {
      refreshUser();
    }
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!username || usernameError !== null) return;
    const formData = new FormData();
    formData.set("username", username);
    startTransition(async () => {
      const result = await updateUserAction(null, formData);
      if ("error" in result) {
        setUsernameError(result.error);
      } else {
        refreshUser();
        onDone();
      }
    });
  };

  const canSubmit = !!username && usernameError === null && !isPending;

  return (
    <form onSubmit={handleSubmit} className="flex flex-col text-sm" noValidate>
      <p className="px-2 pt-2">Welcome to gitdot.</p>
      <div className="flex items-end gap-2 px-2 py-1 border-b border-border">
        <input
          ref={fileInputRef}
          type="file"
          accept="image/jpeg,image/png,image/webp"
          className="hidden"
          onChange={handleFileChange}
        />
        <Tooltip>
          <TooltipTrigger asChild>
            <button
              type="button"
              className="relative size-7 shrink-0 cursor-pointer appearance-none bg-transparent border-none p-0"
              onClick={() => !uploading && fileInputRef.current?.click()}
            >
              <span
                className={`transition-opacity duration-300${uploading ? " opacity-60" : ""}`}
              >
                {user ? (
                  <UserImage user={user} px={28} />
                ) : (
                  <div className="size-7 rounded-full bg-foreground/10" />
                )}
              </span>
              <div
                className={`absolute -inset-0.5 rounded-full border border-transparent border-t-foreground/50 animate-spin transition-opacity duration-300${uploading ? "" : " opacity-0"}`}
              />
            </button>
          </TooltipTrigger>
          <TooltipContent>Upload photo</TooltipContent>
        </Tooltip>
        <input
          name="username"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          className="flex-1 pb-1 ring-0 outline-0 transition-colors duration-150"
          autoFocus
        />
      </div>
      <div className="flex items-center justify-between h-8">
        <div className="flex items-center px-2">
          {(uploadError || usernameError) && (
            <p className="text-xs text-red-500 animate-in fade-in">
              {uploadError ?? usernameError}
            </p>
          )}
        </div>
        <button
          type="submit"
          disabled={!canSubmit}
          className="px-3 h-full text-xs bg-primary text-primary-foreground hover:bg-primary/80 disabled:opacity-50 disabled:cursor-not-allowed transition duration-200"
        >
          Submit
        </button>
      </div>
    </form>
  );
}
