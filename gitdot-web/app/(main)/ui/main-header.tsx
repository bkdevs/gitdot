"use client";

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
import { Files, History, Plus, Search, User } from "lucide-react";
import { useParams, usePathname, useRouter } from "next/navigation";
import { useState } from "react";

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
      <div className="shrink-0 flex flex-row w-full h-9 items-center justify-between border-b bg-sidebar">
        <div className="flex-1 text-sm font-mono flex items-center pl-3">
          {pathLinks}
        </div>
        <div className="flex items-center">
          <NavButton icon={Search} label="Search" onClick={() => {}} />
          <NavButton
            icon={Files}
            label="File"
            onClick={() => window.dispatchEvent(new CustomEvent("openFileSearch"))}
          />
          <NavButton icon={History} label="History" onClick={() => {}} />
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
  label,
  onClick,
}: {
  icon: React.ComponentType<{ className?: string }>;
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      onClick={onClick}
      className="size-9 border-l border-border flex items-center justify-center hover:bg-sidebar-accent transition-colors shrink-0"
    >
      <Icon className="size-4" />
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
          className="size-9 border-l border-border flex items-center justify-center hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent transition-colors shrink-0 outline-none"
        >
          <Icon className="size-4" />
          <span className="sr-only">{label}</span>
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent side="bottom" align="end">{children}</DropdownMenuContent>
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
          className="size-9 border-l border-border flex items-center justify-center hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent transition-colors shrink-0 outline-none"
        >
          <User
            className={cn(
              "size-4 transition-all duration-300",
              user ? "stroke-[2.5]" : "opacity-60",
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
