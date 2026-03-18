"use client";

import { useParams, usePathname, useRouter } from "next/navigation";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { type Shortcut, useShortcuts } from "@/(main)/context/shortcuts";
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
    const active = document.activeElement;
    const el = active?.matches("[data-page-item]")
      ? (active as HTMLElement)
      : document.querySelector<HTMLElement>("[data-page-item]");
    if (!el) return;
    if (el instanceof HTMLAnchorElement) {
      el.click();
    } else {
      el.querySelector<HTMLAnchorElement>("a")?.click();
    }
  }, []);

  const mouseMoved = useRef(false);

  // biome-ignore lint/correctness/useExhaustiveDependencies: pathname triggers reset intentionally
  useEffect(() => {
    mouseMoved.current = false;
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

  const shortcuts = useMemo<Shortcut[]>(
    () => [
      {
        name: "NavDown",
        description: "Next sidebar item",
        keys: ["j"],
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
      {
        name: "NavUp",
        description: "Previous sidebar item",
        keys: ["k"],
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
      {
        name: "ItemDown",
        description: "Next page item",
        keys: ["J"],
        execute: () => {
          const items = Array.from(
            document.querySelectorAll<HTMLElement>("[data-page-item]"),
          );
          if (!items.length) return;
          const activeIdx = items.indexOf(document.activeElement);
          const next = activeIdx === -1 ? 0 : (activeIdx + 1) % items.length;
          items[next].focus();
        },
      },
      {
        name: "ItemUp",
        description: "Previous page item",
        keys: ["K"],
        execute: () => {
          const items = Array.from(
            document.querySelectorAll<HTMLElement>("[data-page-item]"),
          );
          if (!items.length) return;
          const activeIdx = items.indexOf(document.activeElement);
          const prev =
            activeIdx === -1
              ? items.length - 1
              : (activeIdx - 1 + items.length) % items.length;
          items[prev].focus();
        },
      },
      {
        name: "NavPop",
        description: "Go up: cd ..",
        keys: ["h", "Escape"],
        execute: () => navPop(),
      },
      {
        name: "NavPush",
        description: "Go down: cd dir",
        keys: ["l", "Enter"],
        execute: navPush,
      },
      {
        name: "GoTo",
        description: "Open goto dialog",
        keys: ["g"],
        execute: () => window.dispatchEvent(new Event("openGotoDialog")),
      },
      {
        name: "FuzzyFile",
        description: "Open file dialog",
        keys: ["p"],
        execute: () => window.dispatchEvent(new Event("openFileSearch")),
      },
      {
        name: "ToggleLeftSidebar",
        description: "Toggle left sidebar",
        keys: ["["],
        execute: () => window.dispatchEvent(new Event("toggleLeftSidebar")),
      },
      {
        name: "ToggleRightSidebar",
        description: "Toggle right sidebar",
        keys: ["]"],
        execute: () => window.dispatchEvent(new Event("toggleRightSidebar")),
      },
    ],
    [navPop, navPush],
  );

  useShortcuts(shortcuts);
  return null;
}
