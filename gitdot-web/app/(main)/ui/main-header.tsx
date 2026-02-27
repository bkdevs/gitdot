"use client";

import { Search, User } from "lucide-react";
import { useParams, usePathname, useRouter } from "next/navigation";
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

  const inRepo = "owner" in params && "repo" in params;

  return (
    <div className="shrink-0 flex flex-row w-full h-9 items-center justify-between border-b bg-sidebar">
      <div className="flex-1 pl-2 text-sm font-mono flex items-center">
        {pathLinks}
      </div>
      <div className="flex items-center gap-1">
        {inRepo && (
          <button
            type="button"
            aria-label="Open repo files"
            onClick={() =>
              window.dispatchEvent(new CustomEvent("openFileSearch"))
            }
            className="w-5.5 h-5.5 mr-1 rounded-full flex items-center justify-center hover:bg-sidebar-accent outline-none transition-colors"
          >
            <Search className="size-4" />
          </button>
        )}
        <UserDropdown />
      </div>
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
          className="w-5.5 h-5.5 mr-2 rounded-full bg-primary flex items-center justify-center hover:bg-primary/70 data-[state=open]:bg-primary/70 outline-none transition-colors"
        >
          <User
            className={cn(
              "size-4 transition-all duration-300",
              user
                ? "text-primary-foreground stroke-[2.5]"
                : "text-primary-foreground/60",
            )}
          />
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        side="bottom"
        align="end"
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
      >
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
      <DropdownMenuItem
        onClick={() => router.push("/login")}
      >
        Log in
      </DropdownMenuItem>
      <DropdownMenuItem
        onClick={() => router.push("/signup")}
      >
        Sign up
      </DropdownMenuItem>
    </>
  );
}
