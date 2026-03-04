"use client";

import { createContext, useContext, useState } from "react";
import { AuthBlockerDialog } from "../ui/auth-blocker-dialog";
import { useUser } from "./user-provider";

type AuthBlockerContextType = {
  requireAuth: () => boolean;
};
const AuthBlockerContext = createContext<AuthBlockerContextType | null>(null);

/**
 * there are a number of buttons and interactive components that require authentication
 *
 * rather than let those requests go through (and fail on the server), we block them client-side
 * instead based and show a dialog prompting to user to login or signup
 */
export function AuthBlockerProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [open, setOpen] = useState(false);
  const { user } = useUser();

  function requireAuth() {
    if (!user) setOpen(true);
    return !user;
  }

  return (
    <AuthBlockerContext value={{ requireAuth }}>
      {children}
      <AuthBlockerDialog open={open} setOpen={setOpen} />
    </AuthBlockerContext>
  );
}

export function useAuthBlocker(): AuthBlockerContextType {
  const context = useContext(AuthBlockerContext);
  if (!context) {
    throw new Error(
      "useAuthBlocker must be used within an AuthBlockerProvider",
    );
  }

  return context;
}
