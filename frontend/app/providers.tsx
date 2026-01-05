"use client";

import { TooltipProvider } from "@/ui/tooltip";

export function Providers({ children }: { children: React.ReactNode }) {
  return <TooltipProvider delayDuration={0}>{children}</TooltipProvider>;
}
