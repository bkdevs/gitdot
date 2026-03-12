"use client";

import { Files, Plus, Search, User } from "lucide-react";
import { useParams, usePathname, useRouter } from "next/navigation";
import { useState } from "react";
import CreateRepoDialog from "@/(main)/[owner]/ui/create-repo-dialog";
import { useUserContext } from "@/(main)/context";
import { signout } from "@/actions";
import { useMetricsContext } from "@/context/metrics";
import { useAnimateNumber } from "@/hooks/use-animate-number";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu";
import Link from "@/ui/link";
import { cn } from "@/util";

export function MainFooter() {
  const [createRepoOpen, setCreateRepoOpen] = useState(false);
  const { requireAuth } = useUserContext();

  return (
    <>
      <div className="shrink-0 flex w-full h-8 items-center border-t bg-sidebar">
        <div className="text-sm font-mono flex items-center px-2 ml-auto">
          <Breadcrumbs />
          <PageVitals />
        </div>
        <div className="flex items-center">
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

function Breadcrumbs() {
  const pathname = usePathname();
  const params = useParams();
  const pathLinks: React.ReactNode[] = [];
  const segments = pathname.split("/").filter(Boolean);
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

  return pathLinks;
}

function PageVitals() {
  const { FCP, TTFB, CLS, INP } = useMetricsContext();
  const animatedFCP = useAnimateNumber(FCP);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="inline-block w-[5ch] text-center text-xs text-muted-foreground font-mono ml-1.5 hover:text-foreground transition-colors outline-none cursor-pointer"
        >
          {animatedFCP != null ? `${animatedFCP}ms` : "0ms"}
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        side="top"
        align="end"
        sideOffset={12}
        alignOffset={-8}
      >
        <div className="px-2 py-1.5 text-xs font-mono space-y-1">
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">FCP</span>
            <span>{FCP != null ? `${Math.round(FCP)}ms` : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">TTFB</span>
            <span>{TTFB != null ? `${Math.round(TTFB)}ms` : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">CLS</span>
            <span>{CLS != null ? CLS.toFixed(3) : "-"}</span>
          </div>
          <div className="flex justify-between gap-4">
            <span className="text-muted-foreground">INP</span>
            <span>{INP != null ? `${Math.round(INP)}ms` : "-"}</span>
          </div>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
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
