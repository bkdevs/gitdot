import { useRouter } from "next/navigation";
import { useMemo } from "react";
import { useUserContext } from "@/(main)/context/user";
import { signout } from "@/actions";

export type Command = {
  label: string;
  type: "cmd";
  execute: () => void;
};

export function useCommands({
  user,
}: {
  user: { name: string } | null;
}): Command[] {
  const router = useRouter();
  const { refreshUser } = useUserContext();

  return useMemo<Command[]>(() => {
    if (!user) {
      return [
        {
          type: "cmd",
          label: "login",
          execute: () => window.dispatchEvent(new Event("toggleAuthDialog")),
        },
        {
          type: "cmd",
          label: "shortcuts",
          execute: () => window.dispatchEvent(new Event("openShortcuts")),
        },
      ];
    }

    return [
      {
        type: "cmd",
        label: "home",
        execute: () => router.push(`/${user.name}`),
      },
      {
        type: "cmd",
        label: "settings",
        execute: () => window.dispatchEvent(new CustomEvent("openSettings")),
      },
      {
        type: "cmd",
        label: "shortcuts",
        execute: () => window.dispatchEvent(new Event("openShortcuts")),
      },
      {
        type: "cmd",
        label: "logout",
        execute: async () => {
          await signout();
          refreshUser();
        },
      },
      {
        type: "cmd",
        label: "new repo",
        execute: () => window.dispatchEvent(new CustomEvent("openNewRepo")),
      },
      {
        type: "cmd",
        label: "new org",
        execute: () => window.dispatchEvent(new CustomEvent("openNewOrg")),
      },
      {
        type: "cmd",
        label: "migrate repo",
        execute: () => window.dispatchEvent(new CustomEvent("openMigrateRepo")),
      },
    ];
  }, [user, router, refreshUser]);
}
