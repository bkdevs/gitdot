"use client";

import { useMemo } from "react";
// import { useParams, useRouter } from "next/navigation";
import { type ShortcutMap, useShortcuts } from "@/(main)/context/shortcuts";

export function RepoShortcuts() {
  // const router = useRouter();
  // const { owner, repo } = useParams<{ owner: string; repo: string }>();

  const map = useMemo<ShortcutMap>(() => {
    return {
      p: {
        name: "FuzzyFile",
        execute: () => window.dispatchEvent(new Event("openFileSearch")),
      },
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
  }, []);

  useShortcuts(map);
  return null;
}
