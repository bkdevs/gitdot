"use client";

import type {
  GitHubInstallationResource,
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
import {
  getCurrentUserAction,
  listInstallationsAction,
  listUserRepositoriesAction,
} from "@/actions";
import { AuthDialog } from "../ui/auth-dialog";

interface UserContext {
  user: UserResource | null | undefined;
  repositories: RepositoryResource[] | null | undefined;
  memberships: OrganizationMemberResource[] | null | undefined;
  installations: GitHubInstallationResource[] | null | undefined;
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
  const [installations, setInstallations] = useState<
    GitHubInstallationResource[] | null | undefined
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

    if (!current?.user) {
      setRepositories(null);
      setInstallations(null);
      return;
    }

    const [repos, installs] = await Promise.all([
      listUserRepositoriesAction(current.user.name),
      listInstallationsAction(),
    ]);
    setRepositories(repos);
    setInstallations(installs);
  }, []);

  useEffect(() => {
    refreshUser();
  }, [refreshUser]);

  return (
    <UserContext
      value={{
        user,
        repositories,
        memberships,
        installations,
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
