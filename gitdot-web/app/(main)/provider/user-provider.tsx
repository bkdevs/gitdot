"use client";

import type { UserResource } from "gitdot-api";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { getCurrentUserAction } from "@/actions";

interface UserContext {
  user: UserResource | null;
  refreshUser: () => void;
}

const UserContext = createContext<UserContext | null>(null);

/**
 * to enable static-site generation, we have to ensure that _all_ user-specific data is fetched in client-side components
 *
 * luckly this isn't too difficult, but for ergonomics, we do this once at a root-level user provider to avoid repeated data fetching
 * in client-side components
 */
export function UserProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<UserResource | null>(null);

  const refreshUser = useCallback(() => {
    getCurrentUserAction().then(setUser);
  }, []);

  useEffect(() => {
    refreshUser();
  }, [refreshUser]);

  return <UserContext value={{ user, refreshUser }}>{children}</UserContext>;
}

export function useUserContext(): UserContext {
  const context = useContext(UserContext);
  if (!context) {
    throw new Error("useUser must be used within an UserProvider");
  }
  return context;
}
