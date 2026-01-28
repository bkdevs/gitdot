"use client";

import { createContext, useContext } from "react";
import type { UserResponse } from "@/lib/dto";

const UserContext = createContext<UserResponse | null>(null);

export function UserProvider({
  user,
  children,
}: {
  user: UserResponse | null;
  children: React.ReactNode;
}) {
  return <UserContext.Provider value={user}>{children}</UserContext.Provider>;
}

export function useUser() {
  const context = useContext(UserContext);
  return context;
}
