"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useEffect, useState, useTransition } from "react";
import { login } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
export function AuthBlockerDialog({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  const [email, setEmail] = useState("");
  const [sent, setSent] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();

  useEffect(() => {
    if (open) {
      setEmail("");
      setSent(false);
      setError(null);
    }
  }, [open]);

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

  const isValid = email.trim() !== "";

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
        {sent ? (
          <div className="flex flex-col text-sm p-2">
            <p>Success.</p>
            <p className="text-primary/60">
              We sent a link to your email. Click it to continue.
            </p>
          </div>
        ) : (
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
            {error && <p className="text-xs text-red-500 px-2 py-1">{error}</p>}
            <div className="flex items-center justify-end h-9">
              <button
                type="submit"
                disabled={!isValid || isPending}
                className="px-3 py-1.5 h-9 text-xs bg-primary text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isPending ? "Submitting..." : "Log in"}
              </button>
            </div>
          </form>
        )}
      </DialogContent>
    </Dialog>
  );
}
