"use client";

import { useParams, usePathname, useRouter } from "next/navigation";
import { useMemo } from "react";
import { type ShortcutMap, useShortcuts } from "@/(main)/context/shortcuts";

const NAV_PATHS = ["", "files", "commits", "questions", "reviews", "builds"];

export function RepoShortcuts() {
  const router = useRouter();
  const { owner, repo } = useParams<{ owner: string; repo: string }>();
  const pathname = usePathname();

  const map = useMemo<ShortcutMap>(() => {
    const rel = pathname.replace(`/${owner}/${repo}`, "") || "";
    const idx = Math.max(
      0,
      NAV_PATHS.findIndex((p) =>
        p === ""
          ? rel === "" || rel === "/"
          : rel === `/${p}` || rel.startsWith(`/${p}/`),
      ),
    );
    const nav = (i: number) => {
      const p = NAV_PATHS[i];
      router.push(p ? `/${owner}/${repo}/${p}` : `/${owner}/${repo}`);
    };
    return {
      g: {
        name: "navigate-to-home",
        execute: () => router.push(`/${owner}/${repo}`),
      },
      c: {
        name: "navigate-to-commits",
        execute: () => router.push(`/${owner}/${repo}/commits`),
      },
      p: {
        name: "open-file-search",
        execute: () => window.dispatchEvent(new Event("openFileSearch")),
      },
      j: { name: "nav-down", execute: () => nav((idx + 1) % NAV_PATHS.length) },
      k: {
        name: "nav-up",
        execute: () => nav((idx - 1 + NAV_PATHS.length) % NAV_PATHS.length),
      },
    };
  }, [owner, repo, pathname, router]);

  useShortcuts(map);
  return null;
}
