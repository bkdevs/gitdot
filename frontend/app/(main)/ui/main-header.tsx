"use client";

import { LogIn, LogOut, Settings, User, UserRoundPlus } from "lucide-react";
import { useParams, usePathname, useRouter } from "next/navigation";
import { useState } from "react";
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
    <div className="shrink-0 flex flex-row w-full h-9 items-center justify-between border-b bg-sidebar">
      <div className="flex-1 pl-2 text-sm font-mono flex items-center">
        {pathLinks}
      </div>
      <UserDropdown />
    </div>
  );
}

function UserDropdown() {
  const { user } = useUser();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="submit"
          className="w-9 h-9 flex items-center justify-center hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent outline-none"
        >
          <User
            className={cn(
              "size-4 transition-all duration-300",
              user ? "text-foreground stroke-[2.5]" : "text-muted-foreground",
            )}
          />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        side="bottom"
        align="end"
        className="rounded-none min-w-32 p-0"
      >
        {user ? <AuthenticatedMenuItems /> : <UnauthenticatedMenuItems />}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function AuthenticatedMenuItems() {
  const router = useRouter();

  return (
    <>
      <DropdownMenuItem
        onClick={() => router.push("/settings")}
        className="rounded-none px-2 py-1.5 text-sm cursor-pointer"
      >
        <Settings className="size-3" />
        Profile
      </DropdownMenuItem>
      <DropdownMenuItem
        onClick={async () => {
          await signout();
          router.push("/login");
        }}
        className="rounded-none px-2 py-1.5 text-sm cursor-pointer"
      >
        <LogOut className="size-3" />
        Sign out
      </DropdownMenuItem>
    </>
  );
}

function UnauthenticatedMenuItems() {
  const router = useRouter();

  return (
    <>
      <DropdownMenuItem
        onClick={() => router.push("/login")}
        className="rounded-none px-2 py-1.5 text-sm cursor-pointer"
      >
        <LogIn className="size-3" />
        Log in
      </DropdownMenuItem>
      <DropdownMenuItem
        onClick={() => router.push("/signup")}
        className="rounded-none px-2 py-1.5 text-sm cursor-pointer"
      >
        <UserRoundPlus className="size-3" />
        Sign up
      </DropdownMenuItem>
    </>
  );
}
