"use client";

import { openIdb } from "gitdot-dal/client";

export function DatabaseProvider({ children }: { children: React.ReactNode }) {
  openIdb();
  return <>{children}</>;
}
