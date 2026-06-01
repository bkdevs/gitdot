"use client";

import type { CurrentUserResource } from "gitdot-api";
import Image from "next/image";
import { usePathname } from "next/navigation";
import { useCallback, useMemo } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";
import Link from "@/ui/link";
import { cn } from "@/util";

const NAV_LINKS: { label: string; href: string }[] = [
  { label: "/home", href: "/" },
  { label: "/faq", href: "/faq" },
  { label: "/docs", href: "/docs" },
  { label: "/weeks", href: "/weeks" },
  { label: "/releases", href: "/releases" },
];

function isActive(pathname: string, href: string) {
  if (href === "/") return pathname === "/";
  if (href === "/weeks")
    return pathname === "/weeks" || pathname.startsWith("/weeks/");
  return pathname === href;
}

function navClassName(active: boolean) {
  return cn(
    "text-sm font-mono transition-colors",
    active
      ? "text-foreground underline"
      : "text-muted-foreground hover:text-foreground",
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
    <div className="grid grid-cols-1 md:grid-cols-[1fr_min(100%,48rem)_1fr] h-full overflow-hidden">
      <div className="hidden md:flex pl-4 pt-3 flex-col gap-1 items-start">
        <Image
          className="dark:invert"
          src="/gitdot-long-black.svg"
          alt="gitdot logo"
          width={80}
          height={38}
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
            data-nav-item
            onClick={() => window.dispatchEvent(new Event("toggleAuthDialog"))}
            className={cn(navClassName(false), "cursor-pointer text-left")}
          >
            /signup
          </button>
        )}
      </div>

      {children}

      {pathname === "/" && (
        <aside className="hidden lg:flex pt-4 pl-8 pr-4 flex-col gap-8">
          <section className="flex flex-col gap-1">
            <span className="text-sm font-mono text-muted-foreground">
              # this week
            </span>
            <Link
              href="/weeks/20"
              className="text-sm font-medium text-foreground hover:underline"
            >
              Week 20: Build something great.
            </Link>
            <span className="text-xs text-muted-foreground">
              May 24 – May 31, 2026
            </span>
          </section>

          <section className="flex flex-col gap-1">
            <span className="text-sm font-mono text-muted-foreground">
              # next release
            </span>
            <Link
              href="/releases"
              className="text-sm font-medium text-foreground hover:underline"
            >
              v0.4: Reviews, finally.
            </Link>
            <span className="text-xs text-muted-foreground">
              Est. mid-June 2026
            </span>
          </section>
        </aside>
      )}
    </div>
  );
}
