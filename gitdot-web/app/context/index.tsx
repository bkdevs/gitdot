"use client";

import { TooltipProvider } from "@/ui/tooltip";
import { MetricsProvider } from "./metrics";

export function RootProvider({ children }: { children: React.ReactNode }) {
  return (
    <MetricsProvider>
      <TooltipProvider delayDuration={0}>{children}</TooltipProvider>
    </MetricsProvider>
  );
}
