"use client";

import { useActionState, useState } from "react";
import {
  type AuthorizeDeviceActionResult,
  authorizeDeviceAction,
} from "@/actions";
import { cn } from "@/util";

export default function AuthorizeDeviceForm() {
  const [userCode, setUserCode] = useState("");

  const [state, formAction, isPending] = useActionState(
    async (
      _prevState: AuthorizeDeviceActionResult | null,
    ): Promise<AuthorizeDeviceActionResult> => {
      return await authorizeDeviceAction(userCode);
    },
    null,
  );

  const isValidCode = /^[A-Z2-9]{4}-[A-Z2-9]{4}$/.test(userCode.toUpperCase());
  const canSubmit = isValidCode && !isPending;

  if (state?.success) {
    return (
      <div className="flex flex-col text-sm w-sm">
        <p>Device authorized.</p>
      </div>
    );
  }

  return (
    <form action={formAction} className="flex flex-col text-sm w-sm">
      <p className="pb-2">Authorize.</p>

      <input
        type="text"
        name="user_code"
        placeholder="XXXX-XXXX"
        value={userCode}
        onChange={(e) => setUserCode(e.target.value)}
        className="border-border border-b ring-0 outline-0 focus:border-black transition-colors duration-150"
      />

      <div className="flex flex-row mt-2 w-full justify-end">
        <button
          type="submit"
          className={cn(
            "underline transition-all duration-300",
            canSubmit
              ? "cursor-pointer decoration-current"
              : "text-primary/60 cursor-not-allowed decoration-transparent",
          )}
          disabled={!canSubmit}
        >
          {isPending ? "Submitting..." : "Submit."}
        </button>
      </div>

      {state && "error" in state && (
        <p className="text-red-500">{state.error}</p>
      )}
    </form>
  );
}
