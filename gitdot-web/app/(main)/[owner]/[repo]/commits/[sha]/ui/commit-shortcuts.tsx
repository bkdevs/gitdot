"use client";

import { useMemo } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";

export function CommitShortcuts({
  selectedIndex,
  setSelectedIndex,
}: {
  selectedIndex: number;
  setSelectedIndex: (index: number) => void;
}) {
  const shortcuts = useMemo(
    () => [
      {
        name: "NextFile",
        description: "Next file",
        keys: ["j"],
        execute: () => {
          const files = getDiffFiles();
          if (!files.length) return;
          const next = Math.min(selectedIndex + 1, files.length - 1);
          setSelectedIndex(next);
          const target = files[next];
          if (!target) return;
          target.scrollIntoView();
          flashHeader(target);
        },
      },
      {
        name: "PrevFile",
        description: "Previous file",
        keys: ["k"],
        execute: () => {
          const files = getDiffFiles();
          if (!files.length) return;
          const prev = Math.max(selectedIndex - 1, 0);
          setSelectedIndex(prev);
          const target = files[prev];
          if (!target) return;
          target.scrollIntoView();
          flashHeader(target);
        },
      },
    ],
    [selectedIndex, setSelectedIndex],
  );

  useShortcuts(shortcuts);
  return null;
}

function getDiffFiles() {
  return Array.from(document.querySelectorAll<HTMLElement>("[data-diff-file]"));
}

function flashHeader(file: HTMLElement) {
  const path = file.querySelector<HTMLElement>("[data-diff-path]");
  if (!path) return;
  path.removeAttribute("data-diff-path-flash");

  void path.offsetWidth;
  path.setAttribute("data-diff-path-flash", "");
  path.addEventListener(
    "animationend",
    () => path.removeAttribute("data-diff-path-flash"),
    { once: true },
  );
}
