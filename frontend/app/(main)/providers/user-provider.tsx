"use client";

import { getCurrentUserAction } from "@/actions";
import type { UserResponse } from "@/lib/dto";
import { createContext, useContext, useEffect, useState } from "react";

const UserContext = createContext<UserResponse | null>(null);

/**
* to enable static-site generation, we have to ensure that _all_ user-specific data is fetched in client-side components
*
* luckly this isn't too difficult, but for ergonomics, we do this once at a root-level user provider to avoid repeated data fetching
* in client-side components
*/
export function UserProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [user, setUser] = useState<UserResponse | null>(null);
  useEffect(() => {
    getCurrentUserAction().then(setUser);
  }, []);

  return <UserContext.Provider value={user}>{children}</UserContext.Provider>;
}

export function useUser() {
  const context = useContext(UserContext);
  return context;
}
