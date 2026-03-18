"use client";

import { useParams, usePathname, useRouter } from "next/navigation";
import { useMemo } from "react";
import { type ShortcutMap, useShortcuts } from "@/(main)/context/shortcuts";
import { NAV_SECTIONS } from "./ui/sidebar/repo-sidebar-nav";

export function RepoShortcuts() {
  const router = useRouter();
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();

  const map = useMemo<ShortcutMap>(() => {
    const navPop = () => {
      const base = `/${owner}/${repo}`;
      const relPath = pathname.slice(base.length);
      const segments = relPath.split("/").filter(Boolean);

      if (segments.length === 0) return;

      const firstSegment = segments[0];
      const isNavSection = NAV_SECTIONS.has(firstSegment);

      if (!isNavSection && segments.length === 1) {
        // top-level path, e.g., gitdot/gitdot-web -> gitdot/files
        router.push(`${base}/files`);
      } else if (isNavSection && segments.length > 1) {
        router.push(`${base}/${segments.slice(0, -1).join("/")}`);
      }
    };

    return {
      p: {
        name: "FuzzyFile",
        execute: () => window.dispatchEvent(new Event("openFileSearch")),
      },
      h: { name: "NavPop", execute: navPop },
      Escape: { name: "NavPop", execute: navPop },
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
  }, [owner, repo, pathname, router]);

  useShortcuts(map);
  return null;
}
