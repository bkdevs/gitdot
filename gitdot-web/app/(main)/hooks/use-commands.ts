import type { OrganizationResource, RepositoryResource } from "gitdot-api";
import { useRouter } from "next/navigation";
import { useMemo } from "react";
import { signout } from "@/actions";
import { useUserContext } from "@/(main)/context/user";

export type Command = {
  label: string;
  type: "repo" | "org" | "cmd";
  execute: () => void;
};

export function useCommands({
  user,
  repositories,
  organizations,
}: {
  user: { name: string } | null;
  repositories: RepositoryResource[] | null | undefined;
  organizations: OrganizationResource[] | null | undefined;
}): Command[] {
  const router = useRouter();
  const { refreshUser } = useUserContext();

  return useMemo<Command[]>(() => {
    const repos: Command[] = (repositories ?? []).map((r) => ({
      type: "repo",
      label: `${r.owner}/${r.name}`,
      execute: () => router.push(`/${r.owner}/${r.name}`),
    }));

    const orgs: Command[] = (organizations ?? []).map((o) => ({
      type: "org",
      label: o.name,
      execute: () => router.push(`/${o.name}`),
    }));

    const authActions: Command[] = [
      {
        type: "cmd",
        label: "profile",
        execute: () => user && router.push(`/${user.name}`),
      },
      {
        type: "cmd",
        label: "settings",
        execute: () => window.dispatchEvent(new CustomEvent("openSettings")),
      },
      {
        type: "cmd",
        label: "logout",
        execute: async () => { await signout(); refreshUser(); },
      },
    ];

    const unauthActions: Command[] = [
      {
        type: "cmd",
        label: "login",
        execute: () => window.dispatchEvent(new Event("toggleAuthDialog")),
      },
    ];

    const commonActions: Command[] = [
      {
        type: "cmd",
        label: "history",
        execute: () => window.dispatchEvent(new Event("openHistoryDialog")),
      },
      {
        type: "cmd",
        label: "shortcuts",
        execute: () => window.dispatchEvent(new Event("openShortcuts")),
      },
    ];

    return [
      ...repos,
      ...orgs,
      ...(user ? authActions : unauthActions),
      ...commonActions,
    ];
  }, [user, router, repositories, organizations, refreshUser]);
}
