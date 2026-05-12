"use client";

import type {
  OrganizationMemberResource,
  RepositoryResource,
  UserResource,
} from "gitdot-api";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { getCurrentUserAction, listUserRepositoriesAction } from "@/actions";
import { AuthDialog } from "../ui/auth-dialog";

interface UserContext {
  user: UserResource | null | undefined;
  repositories: RepositoryResource[] | null | undefined;
  memberships: OrganizationMemberResource[] | null | undefined;
  refreshUser: () => Promise<void>;
  requireAuth: () => boolean;
}

const UserContext = createContext<UserContext | null>(null);

/**
 * to enable static-site generation, we have to ensure that _all_ user-specific data is fetched in client-side components
 *
 * luckly this isn't too difficult, but for ergonomics, we do this once at a root-level user provider to avoid repeated data fetching
 * in client-side components
 */
export function UserProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<UserResource | null | undefined>(undefined);
  const [memberships, setMemberships] = useState<
    OrganizationMemberResource[] | null | undefined
  >(undefined);
  const [repositories, setRepositories] = useState<
    RepositoryResource[] | null | undefined
  >(undefined);
  const [open, setOpen] = useState(false);

  const requireAuth = useCallback(() => {
    if (!user) setOpen(true);
    return !user;
  }, [user]);

  useEffect(() => {
    const handler = () => setOpen((prev) => !prev);
    window.addEventListener("toggleAuthDialog", handler);
    return () => window.removeEventListener("toggleAuthDialog", handler);
  }, []);

  const refreshUser = useCallback(async () => {
    const current = await getCurrentUserAction();
    setUser(current?.user ?? null);
    setMemberships(current?.memberships ?? null);
  }, []);

  useEffect(() => {
    refreshUser();
  }, [refreshUser]);

  useEffect(() => {
    if (!user) return;
    listUserRepositoriesAction(user.name).then(setRepositories);
  }, [user]);

  return (
    <UserContext
      value={{
        user,
        repositories,
        memberships,
        refreshUser,
        requireAuth,
      }}
    >
      {children}
      <AuthDialog open={open} setOpen={setOpen} />
    </UserContext>
  );
}

export function useUserContext(): UserContext {
  const context = useContext(UserContext);
  if (!context) {
    throw new Error("useUser must be used within an UserProvider");
  }
  return context;
}
