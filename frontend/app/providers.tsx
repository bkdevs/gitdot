"use client";

import { TooltipProvider } from "@/ui/tooltip";

export function Providers({ children }: { children: React.ReactNode }) {
  // consider simply removing this into sidebar... so layouts are meant to be client-side loaded to avoid re-rendering on navigation eh.
  return <TooltipProvider delayDuration={0}>{children}</TooltipProvider>;
}
