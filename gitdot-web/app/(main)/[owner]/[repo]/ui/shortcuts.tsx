"use client";

import { useParams, usePathname } from "next/navigation";
import { useEffect, useLayoutEffect, useMemo, useRef } from "react";
import { type Shortcut, useShortcuts } from "@/(main)/context/shortcuts";

export function RepoShortcuts() {
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();

  const mouseMoved = useRef(false);
  const prevPathname = useRef(pathname);

  // biome-ignore lint/correctness/useExhaustiveDependencies: run on pathname intentionally
  useEffect(() => {
    mouseMoved.current = false;
  }, [pathname]);

  // biome-ignore lint/correctness/useExhaustiveDependencies: run on pathname intentionally
  useLayoutEffect(() => {
    const prev = prevPathname.current;
    prevPathname.current = pathname;

    const isPop = prev.startsWith(`${pathname}/`);
    const base = `/${owner}/${repo}`;
    const isFilesRoot =
      pathname === `${base}/files` &&
      prev.startsWith(`${base}/`) &&
      !prev.slice(base.length + 1).includes("/");

    if (!isPop && !isFilesRoot) return;

    const items = Array.from(
      document.querySelectorAll<HTMLElement>("[data-page-item]"),
    );
    const el = items.find((el) => {
      const anchor =
        el instanceof HTMLAnchorElement ? el : el.querySelector("a");
      return anchor?.pathname === prev;
    });
    if (el) el.focus();
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
    const handleMouseOut = (e: MouseEvent) => {
      if (!mouseMoved.current) return;
      const from = (e.target as HTMLElement).closest<HTMLElement>(
        "[data-page-item]",
      );
      const to = (e.relatedTarget as HTMLElement | null)?.closest<HTMLElement>(
        "[data-page-item]",
      );
      if (from && !to) {
        (document.activeElement as HTMLElement | null)?.blur();
      }
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseover", handleMouseOver);
    document.addEventListener("mouseout", handleMouseOut);
    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseover", handleMouseOver);
      document.removeEventListener("mouseout", handleMouseOut);
    };
  }, []);

  const shortcuts = useMemo<Shortcut[]>(
    () => [
      {
        name: "NavDown",
        description: "Next sidebar item",
        keys: ["J", "Tab"],
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
        keys: ["K", "Shift+Tab"],
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
        keys: ["j"],
        execute: () => {
          const items = Array.from(
            document.querySelectorAll<HTMLElement>("[data-page-item]"),
          );
          if (!items.length) return;
          const activeEl = document.activeElement;
          const activeIdx =
            activeEl instanceof HTMLElement ? items.indexOf(activeEl) : -1;
          const next = activeIdx === -1 ? 0 : (activeIdx + 1) % items.length;
          items[next].focus();
        },
      },
      {
        name: "ItemUp",
        description: "Previous page item",
        keys: ["k"],
        execute: () => {
          const items = Array.from(
            document.querySelectorAll<HTMLElement>("[data-page-item]"),
          );
          if (!items.length) return;
          const activeEl = document.activeElement;
          const activeIdx =
            activeEl instanceof HTMLElement ? items.indexOf(activeEl) : -1;
          const prev =
            activeIdx === -1
              ? items.length - 1
              : (activeIdx - 1 + items.length) % items.length;
          items[prev].focus();
        },
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
    [],
  );

  useShortcuts(shortcuts);
  return null;
}
