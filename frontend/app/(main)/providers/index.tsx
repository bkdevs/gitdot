"use client";

import type { UserResponse } from "@/lib/dto";
import { TooltipProvider } from "@/ui/tooltip";
import { UserProvider } from "./user-provider";

export function Providers({
  user,
  children,
}: {
  user: UserResponse | null;
  children: React.ReactNode;
}) {
  return (
    <UserProvider user={user}>
      <TooltipProvider delayDuration={0}>{children}</TooltipProvider>
    </UserProvider>
  );
}
