"use client";

import { TooltipProvider } from "@/ui/tooltip";
import { AuthBlockerProvider } from "./auth-blocker-provider";
import { MetricsProvider } from "./metrics-provider";
import { UserProvider } from "./user-provider";

export function MainProvider({ children }: { children: React.ReactNode }) {
  return (
    <MetricsProvider>
      <UserProvider>
        <AuthBlockerProvider>
          <TooltipProvider delayDuration={0}>{children}</TooltipProvider>
        </AuthBlockerProvider>
      </UserProvider>
    </MetricsProvider>
  );
}
