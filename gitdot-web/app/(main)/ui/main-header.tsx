"use client";

import { Circle, Files, Plus, Search } from "lucide-react";
import { useParams, usePathname, useRouter } from "next/navigation";
import { useState } from "react";
import CreateRepoDialog from "@/(main)/[owner]/ui/create-repo-dialog";
import { useAuthBlocker } from "@/(main)/providers/auth-blocker-provider";
import { useUser } from "@/(main)/providers/user-provider";
import { signout } from "@/actions";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { cn } from "@/util";

export function MainHeader() {
  const pathname = usePathname();
  const params = useParams();
  const segments = pathname.split("/").filter(Boolean);
  const pathLinks: React.ReactNode[] = [];
  const [createRepoOpen, setCreateRepoOpen] = useState(false);
  const { requireAuth } = useAuthBlocker();

  segments.forEach((segment, index) => {
    let path = `/${segments.slice(0, index + 1).join("/")}`;
    if ("filePath" in params && index === 1) {
      path = `${path}/files`;
    }

    if (index > 0) {
      pathLinks.push(<span key={`sep-${segment}`}>/</span>);
    }
    pathLinks.push(
      <Link
        className="hover:underline"
        href={path}
        key={`segment-${segment}`}
        prefetch={true}
      >
        {segment}
      </Link>,
    );
  });

  return (
    <>
      <div className="shrink-0 grid grid-cols-[1fr_auto_1fr] w-full h-8 items-center border-b bg-sidebar">
        <div />
        <div className="text-sm font-mono flex items-center gap-0.5">
          {pathLinks}
        </div>
        <div className="flex items-center justify-end">
          <NavButton icon={Search} label="Search" onClick={() => {}} />
          <NavButton
            icon={Files}
            label="File"
            onClick={() =>
              window.dispatchEvent(new CustomEvent("openFileSearch"))
            }
          />
          <DropdownNavButton icon={Plus} label="Create">
            <DropdownMenuItem
              onClick={() => {
                if (requireAuth()) return null;
                setCreateRepoOpen(true);
              }}
            >
              New repo
            </DropdownMenuItem>
          </DropdownNavButton>
          <UserDropdown />
        </div>
      </div>
      <CreateRepoDialog open={createRepoOpen} setOpen={setCreateRepoOpen} />
    </>
  );
}

function NavButton({
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

function DropdownNavButton({
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
  const { user } = useUser();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="size-8 border-l border-border flex items-center justify-center hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent transition-colors shrink-0 outline-none"
        >
          <Circle
            className={cn(
              "size-2 transition-all duration-300",
              user ? "fill-current stroke-current" : "fill-transparent",
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
      <DropdownMenuItem onClick={() => router.push("/settings")}>
        Profile
      </DropdownMenuItem>
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
  const router = useRouter();

  return (
    <>
      <DropdownMenuItem onClick={() => router.push("/login")}>
        Log in
      </DropdownMenuItem>
      <DropdownMenuItem onClick={() => router.push("/signup")}>
        Sign up
      </DropdownMenuItem>
    </>
  );
}
