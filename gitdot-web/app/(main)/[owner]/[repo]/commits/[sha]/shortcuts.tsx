"use client";

import { useMemo } from "react";
import { useShortcuts } from "@/(main)/context/shortcuts";

function getDiffFiles() {
  return Array.from(
    document.querySelectorAll<HTMLElement>("[data-diff-file]"),
  );
}

function getCurrentIndex(files: HTMLElement[]) {
  let current = 0;
  for (let i = 0; i < files.length; i++) {
    if (files[i].getBoundingClientRect().top <= 4) {
      current = i + 1;
    }
  }
  return current;
}

function flashHeader(file: HTMLElement) {
  const path = file.querySelector<HTMLElement>("[data-diff-path]");
  if (!path) return;
  path.removeAttribute("data-diff-path-flash");

  void path.offsetWidth;
  path.setAttribute("data-diff-path-flash", "");
  path.addEventListener("animationend", () => path.removeAttribute("data-diff-path-flash"), {
    once: true,
  });
}

export function CommitShortcuts() {
  const shortcuts = useMemo(
    () => [
      {
        name: "NextFile",
        description: "Next file",
        keys: ["j"],
        execute: () => {
          const files = getDiffFiles();
          if (!files.length) return;
          const target = files[getCurrentIndex(files)];
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
          const current = getCurrentIndex(files);

          if (current === 0) return;
          if (current === 1) {
            document
              .querySelector<HTMLElement>("[data-commit-top]")
              ?.scrollIntoView();
          } else {
            const target = files[current - 1];
            target.scrollIntoView();
            flashHeader(target);
          }
        },
      },
    ],
    [],
  );

  useShortcuts(shortcuts);
  return null;
}
