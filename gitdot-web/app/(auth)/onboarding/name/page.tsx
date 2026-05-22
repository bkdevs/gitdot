"use client";

import { useRouter } from "next/navigation";
import {
  type FormEvent,
  useEffect,
  useRef,
  useState,
  useTransition,
} from "react";
import { updateUserAction, validateUsername } from "@/actions";
import { cn } from "@/util";

export default function Page() {
  const [username, setUsername] = useState("");
  const [validationError, setValidationError] = useState<string | null>(null);
  const [isChecking, setIsChecking] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [isPending, startTransition] = useTransition();
  const [visible, setVisible] = useState(false);

  const requestIdRef = useRef(0);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const router = useRouter();

  useEffect(() => {
    const t = setTimeout(() => setVisible(true), 50);
    return () => clearTimeout(t);
  }, []);

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

  function handleSubmit(event: FormEvent) {
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
      router.push("/onboarding/github");
    });
  }

  let footerMessage = "";
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
    <div className="min-h-screen flex items-center justify-center pb-[10vh]">
      <form
        onSubmit={handleSubmit}
        className="flex flex-col text-sm w-sm transition-opacity duration-1000"
        style={{ opacity: visible ? 1 : 0 }}
        noValidate
      >
        <p className="pb-2">Choose username.</p>
        <input
          type="text"
          name="username"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoFocus
          autoComplete="off"
          spellCheck={false}
          disabled={isPending}
          className="border-border border-b mb-2 ring-0 outline-0 focus:border-black transition-colors duration-1000"
        />
        <div className="flex flex-row w-full justify-between items-baseline">
          <p className={cn("text-xs", footerColor)}>{footerMessage}</p>
          <button
            type="submit"
            disabled={!canSubmit}
            className={cn(
              "cursor-pointer underline transition-colors duration-300 disabled:cursor-not-allowed",
              canSubmit
                ? "text-foreground decoration-current"
                : "text-muted-foreground decoration-transparent",
            )}
          >
            Next.
          </button>
        </div>
      </form>
    </div>
  );
}
