"use client";

import { TooltipProvider } from "@/ui/tooltip";
import { UserProvider } from "./user-provider";

export function Providers({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <UserProvider>
      <TooltipProvider delayDuration={0}>{children}</TooltipProvider>
    </UserProvider>
  );
}
