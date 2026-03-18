"use client";

import { useParams, usePathname, useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo } from "react";
import { type ShortcutMap, useShortcuts } from "@/(main)/context/shortcuts";
import { NAV_SECTIONS } from "./ui/sidebar/repo-sidebar-nav";

export function RepoShortcuts() {
  const router = useRouter();
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();

  const navPop = useCallback(() => {
    const base = `/${owner}/${repo}`;
    const relPath = pathname.slice(base.length);
    const segments = relPath.split("/").filter(Boolean);

    if (segments.length === 0) return false;

    const firstSegment = segments[0];
    const isNavSection = NAV_SECTIONS.has(firstSegment);

    if (!isNavSection && segments.length === 1) {
      // top-level path, e.g., gitdot/gitdot-web -> gitdot/files
      router.push(`${base}/files`);
    } else if (segments.length > 1) {
      router.push(`${base}/${segments.slice(0, -1).join("/")}`);
    }
    return true;
  }, [owner, repo, pathname, router]);

  const navPush = useCallback(() => {
    // take focus or first element if no focus present
    const el = document.activeElement?.matches("[data-page-item]")
      ? (document.activeElement as HTMLElement)
      : document.querySelector<HTMLElement>("[data-page-item]");

    if (!el) return;

    if (el instanceof HTMLAnchorElement) {
      el.click();
    } else {
      el.querySelector<HTMLAnchorElement>("a")?.click();
    }
  }, []);

  // register a global mouseover that focuses the hovered data-page-item
  useEffect(() => {
    const handleMouseOver = (e: MouseEvent) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>(
        "[data-page-item]",
      );
      el?.focus();
    };
    document.addEventListener("mouseover", handleMouseOver);
    return () => document.removeEventListener("mouseover", handleMouseOver);
  }, []);

  // override the browser back button to act like nav pop rather than back / forth
  useEffect(() => {
    history.pushState({ navIntercepted: true }, "");

    const handlePopState = () => {
      const didNavigate = navPop();
      if (!didNavigate) return;
    };

    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  }, [navPop]);

  const map = useMemo<ShortcutMap>(() => {
    return {
      p: {
        name: "FuzzyFile",
        execute: () => window.dispatchEvent(new Event("openFileSearch")),
      },
      h: { name: "NavPop", execute: () => navPop() },
      Escape: { name: "NavPop", execute: () => navPop() },
      l: { name: "NavPush", execute: navPush },
      Enter: { name: "NavPush", execute: navPush },
      j: {
        name: "NavDown",
        execute: () => {
          const items = Array.from(
            document.querySelectorAll<HTMLElement>("[data-sidebar-item]"),
          );
          if (!items.length) return;
          const activeIdx = items.findIndex(
            (el) => el.dataset.sidebarItemActive === "true",
          );
          if (activeIdx === -1) return;

          items[(activeIdx + 1) % items.length].click();
        },
      },
      k: {
        name: "NavUp",
        execute: () => {
          const items = Array.from(
            document.querySelectorAll<HTMLElement>("[data-sidebar-item]"),
          );
          if (!items.length) return;
          const activeIdx = items.findIndex(
            (el) => el.dataset.sidebarItemActive === "true",
          );
          if (activeIdx === -1) return;

          items[(activeIdx - 1 + items.length) % items.length].click();
        },
      },
    };
  }, [navPop, navPush]);

  useShortcuts(map);
  return null;
}
