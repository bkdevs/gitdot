"use client";

import { Settings, User } from "lucide-react";
import { useRouter } from "next/navigation";
import { useUserContext } from "@/(main)/context/user";
import { signout } from "@/actions";
import { QuestionMark } from "@/icons";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { cn } from "@/util";

export function MainToolbar() {
  return (
    <div className="flex items-center">
      <UserDropdown />
      <ToolbarButton
        icon={Settings}
        label="Settings"
        onClick={() => window.dispatchEvent(new CustomEvent("openSettings"))}
      />
      <ToolbarButton
        icon={QuestionMark}
        label="Shortcuts"
        onClick={() => window.dispatchEvent(new CustomEvent("openShortcuts"))}
      />
    </div>
  );
}

function ToolbarButton({
  icon: Icon,
  iconClassName,
  label,
  onClick,
}: {
  icon: React.ComponentType<{ className?: string }>;
  iconClassName?: string;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="size-8 border-l border-border flex items-center justify-center hover:bg-sidebar-accent transition-colors shrink-0"
    >
      <Icon className={cn("size-4", iconClassName)} />
      <span className="sr-only">{label}</span>
    </button>
  );
}

function UserDropdown() {
  const { user } = useUserContext();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="size-8 border-l border-border flex items-center justify-center hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent transition-colors shrink-0 outline-none"
        >
          <User
            className={cn(
              "size-4 transition-all duration-300",
              user ? "" : "opacity-60",
            )}
          />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent side="bottom" align="end">
        {user ? <AuthenticatedMenuItems /> : <UnauthenticatedMenuItems />}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function AuthenticatedMenuItems() {
  const router = useRouter();

  return (
    <>
      <Link href="/settings">
        <DropdownMenuItem>Profile</DropdownMenuItem>
      </Link>
      <DropdownMenuItem
        onClick={async () => {
          await signout();
          router.push("/login");
        }}
      >
        Sign out
      </DropdownMenuItem>
    </>
  );
}

function UnauthenticatedMenuItems() {
  return (
    <>
      <Link href="/login">
        <DropdownMenuItem>Log in</DropdownMenuItem>
      </Link>
      <Link href="/signup">
        <DropdownMenuItem>Sign up</DropdownMenuItem>
      </Link>
    </>
  );
}
