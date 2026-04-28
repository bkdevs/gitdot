"use client";

import { Toaster } from "sonner";

export { toast } from "sonner";

export function ToasterProvider({ children }: { children: React.ReactNode }) {
  return (
    <>
      {children}
      <Toaster position="bottom-right" theme="system" />
    </>
  );
}
