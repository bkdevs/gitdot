"use client";

import { AuthBlockerProvider } from "./auth-blocker-provider";
import { UserProvider } from "./user-provider";

export function MainProvider({ children }: { children: React.ReactNode }) {
  return (
    <UserProvider>
      <AuthBlockerProvider>{children}</AuthBlockerProvider>
    </UserProvider>
  );
}
