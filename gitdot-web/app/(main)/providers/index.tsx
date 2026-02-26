"use client";

import { TooltipProvider } from "@/ui/tooltip";
import { AuthBlockerProvider } from "./auth-blocker-provider";
import { UserProvider } from "./user-provider";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <UserProvider>
      <AuthBlockerProvider>
        <TooltipProvider delayDuration={0}>{children}</TooltipProvider>
      </AuthBlockerProvider>
    </UserProvider>
  );
}
