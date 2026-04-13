"use client";

import type {
  OrganizationResource,
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
import {
  getCurrentUserAction,
  listUserOrganizationsAction,
  listUserRepositoriesAction,
} from "@/actions";
import { AuthDialog } from "../ui/auth-dialog";

interface UserContext {
  user: UserResource | null | undefined;
  repositories: RepositoryResource[] | null | undefined;
  organizations: OrganizationResource[] | null | undefined;
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
  const [repositories, setRepositories] = useState<
    RepositoryResource[] | null | undefined
  >(undefined);
  const [organizations, setOrganizations] = useState<
    OrganizationResource[] | null | undefined
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
    setUser(await getCurrentUserAction());
  }, []);

  useEffect(() => {
    refreshUser();
  }, [refreshUser]);

  useEffect(() => {
    if (!user) return;
    Promise.all([
      listUserRepositoriesAction(user.name),
      listUserOrganizationsAction(user.name),
    ]).then(([repos, orgs]) => {
      setRepositories(repos);
      setOrganizations(orgs);
    });
  }, [user]);

  return (
    <UserContext
      value={{ user, repositories, organizations, refreshUser, requireAuth }}
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
