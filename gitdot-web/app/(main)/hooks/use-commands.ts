import type {
  OrganizationMemberResource,
  RepositoryResource,
} from "gitdot-api";
import { useRouter } from "next/navigation";
import { useMemo } from "react";
import { useUserContext } from "@/(main)/context/user";
import { signout } from "@/actions";

export type Command = {
  label: string;
  type: "repo" | "org" | "cmd";
  execute: () => void;
};

export function useCommands({
  user,
  repositories,
  memberships,
}: {
  user: { name: string } | null;
  repositories: RepositoryResource[] | null | undefined;
  memberships: OrganizationMemberResource[] | null | undefined;
}): Command[] {
  const router = useRouter();
  const { refreshUser } = useUserContext();

  return useMemo<Command[]>(() => {
    const repos: Command[] = (repositories ?? []).map((r) => ({
      type: "repo",
      label: `${r.owner}/${r.name}`,
      execute: () => router.push(`/${r.owner}/${r.name}`),
    }));

    const orgs: Command[] = (memberships ?? []).map((m) => ({
      type: "org",
      label: m.org_name,
      execute: () => router.push(`/${m.org_name}`),
    }));

    const authActions: Command[] = [
      {
        type: "cmd",
        label: "profile",
        execute: () => user && router.push(`/${user.name}`),
      },
      {
        type: "cmd",
        label: "new-repo",
        execute: () => window.dispatchEvent(new CustomEvent("openNewRepo")),
      },
      {
        type: "cmd",
        label: "new-org",
        execute: () => window.dispatchEvent(new CustomEvent("openNewOrg")),
      },
      {
        type: "cmd",
        label: "settings",
        execute: () => window.dispatchEvent(new CustomEvent("openSettings")),
      },
      {
        type: "cmd",
        label: "logout",
        execute: async () => {
          await signout();
          refreshUser();
        },
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
      ...(user ? repos : []),
      ...(user ? orgs : []),
      ...(user ? authActions : unauthActions),
      ...commonActions,
    ];
  }, [user, router, repositories, memberships, refreshUser]);
}
