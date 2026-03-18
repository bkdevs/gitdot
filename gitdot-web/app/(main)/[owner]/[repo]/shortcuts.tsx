"use client";

import { useParams, usePathname, useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useRef } from "react";
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
    const el = document.activeElement;
    if (!el?.matches("[data-page-item]")) return;
    if (el instanceof HTMLAnchorElement) {
      el.click();
    } else {
      (el as HTMLElement).querySelector<HTMLAnchorElement>("a")?.click();
    }
  }, []);

  const mouseMoved = useRef(false);

  // biome-ignore lint/correctness/useExhaustiveDependencies: pathname triggers reset intentionally
  useEffect(() => {
    mouseMoved.current = false;
  }, [pathname]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: pathname triggers autofocus intentionally
  useEffect(() => {
    const existing = document.querySelector<HTMLElement>("[data-page-item]");
    if (existing) {
      if (!mouseMoved.current) existing.focus();
      return;
    }

    const observer = new MutationObserver(() => {
      const el = document.querySelector<HTMLElement>("[data-page-item]");
      if (!el) return;

      observer.disconnect();
      if (!mouseMoved.current) el.focus();
    });

    observer.observe(document.body, { childList: true, subtree: true });
    return () => observer.disconnect();
  }, [pathname]);

  // register a global mouseover that focuses the hovered data-page-item
  useEffect(() => {
    const handleMouseMove = () => {
      mouseMoved.current = true;
    };
    const handleMouseOver = (e: MouseEvent) => {
      if (!mouseMoved.current) return;
      const el = (e.target as HTMLElement).closest<HTMLElement>(
        "[data-page-item]",
      );
      el?.focus();
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseover", handleMouseOver);
    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseover", handleMouseOver);
    };
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
