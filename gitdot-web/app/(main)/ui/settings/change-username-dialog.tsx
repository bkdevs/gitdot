"use client";

import { VisuallyHidden } from "@radix-ui/react-visually-hidden";
import { useRouter } from "next/navigation";
import { useEffect, useRef, useState, useTransition } from "react";
import { toast } from "@/(main)/provider/toaster";
import { useUserContext } from "@/(main)/provider/user";
import { updateUserAction, validateUsername } from "@/actions";
import { Dialog, DialogContent, DialogTitle } from "@/ui/dialog";
import { cn } from "@/util";

export function ChangeUsernameDialog({
  open,
  setOpen,
  setSettingsOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  setSettingsOpen: (open: boolean) => void;
}) {
  const { refreshUser } = useUserContext();
  const router = useRouter();
  const [username, setUsername] = useState("");
  const [validationError, setValidationError] = useState<string | null>(null);
  const [isChecking, setIsChecking] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const requestIdRef = useRef(0);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (open) {
      setUsername("");
      setValidationError(null);
      setSubmitError(null);
      setIsChecking(false);
      requestIdRef.current = 0;
    }
  }, [open]);

  useEffect(() => {
    if (username === "") {
      setValidationError(null);
      setIsChecking(false);
      if (debounceRef.current) clearTimeout(debounceRef.current);
      return;
    }
    setIsChecking(true);
    if (debounceRef.current) clearTimeout(debounceRef.current);
    const myId = ++requestIdRef.current;
    debounceRef.current = setTimeout(async () => {
      const result = await validateUsername(username);
      if (myId !== requestIdRef.current) return;
      setValidationError(result);
      setIsChecking(false);
    }, 300);
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, [username]);

  const canSubmit =
    username.length > 0 &&
    !isChecking &&
    validationError === null &&
    !isPending;

  function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (!canSubmit) return;
    setSubmitError(null);
    startTransition(async () => {
      const fd = new FormData();
      fd.set("username", username);
      const result = await updateUserAction(null, fd);
      if ("error" in result) {
        setSubmitError(result.error);
        return;
      }
      await refreshUser();
      setOpen(false);
      setSettingsOpen(false);
      router.push(`/${username}`);
      await new Promise((resolve) => setTimeout(resolve, 600));
      toast.success("Username changed");
    });
  }

  let footerMessage: string = "Change username";
  let footerColor = "text-muted-foreground";
  if (submitError) {
    footerMessage = submitError;
    footerColor = "text-red-500";
  } else if (!isChecking && validationError) {
    footerMessage = validationError;
    footerColor = "text-red-500";
  } else if (isChecking) {
    footerMessage = "checking...";
  } else if (username.length > 0 && validationError === null) {
    footerMessage = "Username available";
    footerColor = "text-green-500";
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent
        className="p-0 overflow-hidden w-96"
        animations
        showOverlay
      >
        <VisuallyHidden>
          <DialogTitle>Change username</DialogTitle>
        </VisuallyHidden>
        <form onSubmit={handleSubmit}>
          <input
            type="text"
            name="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder={"Username..."}
            className="w-full p-2 bg-background outline-none text-sm"
            disabled={isPending}
            autoFocus
            autoComplete="off"
            spellCheck={false}
          />
          <div className="flex items-center justify-between h-7 border-t border-border">
            <div className="flex items-center px-2 min-w-0">
              <p className={cn("text-xs truncate", footerColor)}>
                {footerMessage}
              </p>
            </div>
            <div className="flex items-center h-full">
              <button
                type="button"
                onClick={() => setOpen(false)}
                className="flex items-center px-2 h-full text-xs border-l border-border hover:bg-accent/50 transition-colors"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!canSubmit}
                className="flex items-center px-3 h-full text-xs bg-primary text-foreground border-l border-primary enabled:hover:opacity-90 disabled:opacity-60 transition-opacity disabled:cursor-not-allowed"
              >
                {isPending ? "Saving..." : "Save"}
              </button>
            </div>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
