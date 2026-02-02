"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useState, useTransition } from "react";
import { type AuthActionResult, login, signup } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { useUser } from "../providers/user-provider";

export function AuthBlockerDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [mode, setMode] = useState<"login" | "signup">("login");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  const { refreshUser } = useUser();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    const formData = new FormData();
    formData.append("email", email);
    formData.append("password", password);

    startTransition(async () => {
      const action = mode === "login" ? login : signup;
      const result: AuthActionResult = await action(formData);

      if (result.success) {
        refreshUser();
        setOpen(false);
        setEmail("");
        setPassword("");
        setMode("login");
      } else {
        setError(result.error);
      }
    });
  };

  const toggleMode = () => {
    setMode(mode === "login" ? "signup" : "login");
    setError(null);
  };

  const isValid = email.trim() !== "" && password.trim() !== "";

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="max-w-md min-w-md border-black rounded-xs shadow-2xl top-[35%] p-0 overflow-hidden"
        animations={true}
        showOverlay={true}
      >
        <VisuallyHidden>
          <DialogTitle>Authenticate</DialogTitle>
        </VisuallyHidden>
        <form onSubmit={handleSubmit}>
          <p className="p-2 text-sm border-b border-border">
            Please authenticate to proceed.
          </p>
          <input
          type="email"
          name="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="w-full p-2 text-sm bg-background outline-none border-b border-border"
          disabled={isPending}
          autoFocus
          />
          <input
          type="password"
          name="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="w-full p-2 text-sm bg-background outline-none border-b border-border"
          disabled={isPending}
          />
          {error && <p className="text-xs text-red-500 px-2 py-1">{error}</p>}
          <div className="flex items-center justify-end h-9">
            {/* disable the sign up button for now */}
            {/*
              <button
              type="button"
              onClick={toggleMode}
              className="group px-3 py-1.5 h-9 text-xs text-muted-foreground"
              >
              {mode === "login" ? (
              <>
              Don&apos;t have an account?{" "}
              <span className="group-hover:text-foreground">Sign up</span>
              </>
              ) : (
              <>
              Already have an account?{" "}
              <span className="group-hover:text-foreground">Log in</span>
              </>
              )}
              </button>
              */}
            <button
              type="submit"
              disabled={!isValid || isPending}
              className="px-3 py-1.5 h-9 text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending
                ? mode === "login"
                ? "Logging in..."
                : "Signing up..."
                : mode === "login"
                ? "Log in"
                : "Sign up"}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
