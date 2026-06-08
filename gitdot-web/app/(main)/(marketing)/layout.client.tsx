"use client";

import type { CurrentUserResource } from "gitdot-api";
import Image from "next/image";
import { usePathname } from "next/navigation";
import { useCallback, useMemo, useState } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import { useUserContext } from "@/(main)/context/user";
import { Dialog, DialogContent, DialogTitle, DialogTrigger } from "@/ui/dialog";
import Link from "@/ui/link";
import { cn } from "@/util";

const NAV_LINKS: { label: string; href: string }[] = [
  { label: "/home", href: "/" },
  { label: "/faq", href: "/faq" },
  { label: "/weeks", href: "/weeks" },
  { label: "/designs", href: "/designs" },
  { label: "/releases", href: "/releases" },
];

function isActive(pathname: string, href: string) {
  if (href === "/") return pathname === "/";
  return pathname === href || pathname.startsWith(`${href}/`);
}

function navClassName(active: boolean) {
  return cn(
    "w-full py-0.5 text-sm font-mono cursor-pointer underline decoration-transparent outline-none ring-0",
    active
      ? "text-foreground decoration-current"
      : "text-muted-foreground hover:text-foreground hover:decoration-current",
  );
}

export function LayoutClient({
  user,
  children,
}: {
  user: CurrentUserResource | null;
  children: React.ReactNode;
}) {
  const pathname = usePathname();
  const { openAuthDialog } = useUserContext();
  const [mobileDialogOpen, setMobileDialogOpen] = useState(false);

  const cycle = useCallback((delta: number) => {
    const items = Array.from(
      document.querySelectorAll<HTMLElement>("[data-nav-item]"),
    );
    if (!items.length) return;
    const activeIdx = items.findIndex(
      (el) => el.dataset.navItemActive === "true",
    );
    if (activeIdx === -1) return;
    items[(activeIdx + delta + items.length) % items.length].click();
  }, []);

  const focusItem = useCallback((delta: number) => {
    const items = Array.from(
      document.querySelectorAll<HTMLElement>("[data-page-item]"),
    );
    if (!items.length) return;
    const activeEl = document.activeElement;
    const activeIdx =
      activeEl instanceof HTMLElement ? items.indexOf(activeEl) : -1;
    const next =
      activeIdx === -1
        ? delta > 0
          ? 0
          : items.length - 1
        : (activeIdx + delta + items.length) % items.length;
    items[next].focus();
  }, []);

  useShortcuts(
    useMemo(
      () => [
        {
          name: "Next",
          description: "Next nav item",
          keys: ["Tab"],
          execute: () => cycle(1),
        },
        {
          name: "Previous",
          description: "Previous nav item",
          keys: ["Shift+Tab"],
          execute: () => cycle(-1),
        },
        {
          name: "ItemDown",
          description: "Next page item",
          keys: ["j"],
          execute: () => focusItem(1),
        },
        {
          name: "ItemUp",
          description: "Previous page item",
          keys: ["k"],
          execute: () => focusItem(-1),
        },
      ],
      [cycle, focusItem],
    ),
  );

  return (
    <div className="h-full overflow-y-auto scrollbar-none outline-none flex flex-col">
      {/* mobile nav */}
      <Dialog open={mobileDialogOpen} onOpenChange={setMobileDialogOpen}>
        <DialogTrigger asChild>
          <button
            type="button"
            aria-label="Open menu"
            className="px-3 pt-4 flex items-center justify-center shrink-0 w-fit md:hidden"
          >
            <span className="size-4 rounded-full bg-foreground" />
          </button>
        </DialogTrigger>
        <DialogContent className="max-w-xs! gap-1 p-4" showOverlay={false}>
          <DialogTitle className="sr-only">Menu</DialogTitle>
          {NAV_LINKS.map((link) => {
            const active = isActive(pathname, link.href);
            return (
              <Link
                key={link.href}
                href={link.href}
                onClick={() => setMobileDialogOpen(false)}
                className={navClassName(active)}
              >
                {link.label}
              </Link>
            );
          })}
        </DialogContent>
      </Dialog>

      {/* desktop nav */}
      <div className="grid grid-cols-1 md:grid-cols-[1fr_min(100%,48rem)_1fr] flex-1 min-h-0">
        <div className="hidden md:flex pr-4 pt-4 flex-col items-end text-right sticky top-0 self-start">
          <Image
            className="dark:invert"
            src="/gitdot-long-black.svg"
            alt="gitdot logo"
            width={64}
            height={30}
            priority
          />
          {NAV_LINKS.map((link) => {
            const active = isActive(pathname, link.href);
            return (
              <Link
                key={link.href}
                href={link.href}
                data-nav-item
                data-nav-item-active={active ? "true" : undefined}
                className={navClassName(active)}
              >
                {link.label}
              </Link>
            );
          })}
          {!user && (
            <button
              type="button"
              onClick={() => openAuthDialog("signup")}
              className={cn(navClassName(false), "cursor-pointer text-right")}
            >
              /signup
            </button>
          )}
        </div>

        {children}
      </div>
    </div>
  );
}
