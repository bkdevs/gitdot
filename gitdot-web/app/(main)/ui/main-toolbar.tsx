"use client";

import { Files, Plus, Search, User } from "lucide-react";
import { useRouter } from "next/navigation";
import { useState } from "react";
import CreateRepoDialog from "@/(main)/[owner]/ui/create-repo-dialog";
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
  const [createRepoOpen, setCreateRepoOpen] = useState(false);
  const { requireAuth } = useUserContext();

  return (
    <div className="flex items-center">
      <ToolbarButton icon={Search} label="Search" onClick={() => {}} />
      <ToolbarButton
        icon={Files}
        label="File"
        onClick={() => window.dispatchEvent(new CustomEvent("openFileSearch"))}
      />
      <DropdownToolbarButton icon={Plus} label="Create">
        <DropdownMenuItem
          onClick={() => {
            if (requireAuth()) return null;
            setCreateRepoOpen(true);
          }}
        >
          New repo
        </DropdownMenuItem>
      </DropdownToolbarButton>
      <UserDropdown />
      <ToolbarButton
        icon={QuestionMark}
        label="Shortcuts"
        onClick={() => window.dispatchEvent(new CustomEvent("openShortcuts"))}
      />

      <CreateRepoDialog open={createRepoOpen} setOpen={setCreateRepoOpen} />
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

function DropdownToolbarButton({
  icon: Icon,
  label,
  children,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  children: React.ReactNode;
}) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="size-8 border-l border-border flex items-center justify-center hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent transition-colors shrink-0 outline-none"
        >
          <Icon className="size-4" />
          <span className="sr-only">{label}</span>
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent side="bottom" align="end">
        {children}
      </DropdownMenuContent>
    </DropdownMenu>
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
