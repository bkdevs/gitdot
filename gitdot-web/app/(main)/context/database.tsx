"use client";

import { openIdb } from "@/db";

export function DatabaseProvider({ children }: { children: React.ReactNode }) {
  openIdb();
  return <>{children}</>;
}
