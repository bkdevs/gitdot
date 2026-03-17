"use client";

import { useParams, useRouter } from "next/navigation";
import { useMemo } from "react";
import { type ShortcutMap, useShortcuts } from "@/(main)/context/shortcuts";

export function RepoShortcuts() {
  const router = useRouter();
  const { owner, repo } = useParams<{ owner: string; repo: string }>();

  const map = useMemo<ShortcutMap>(
    () => ({
      g: {
        name: "navigate-to-files",
        execute: () => router.push(`/${owner}/${repo}`),
      },
      c: {
        name: "navigate-to-commits",
        execute: () => router.push(`/${owner}/${repo}/commits`),
      },
      j: {
        name: "navigate-down",
        execute: () => console.log("navigate-down"),
      },
      k: {
        name: "navigate-up",
        execute: () => console.log("navigate-up"),
      },
    }),
    [owner, repo, router],
  );

  useShortcuts(map);
  return null;
}
